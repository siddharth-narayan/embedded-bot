use core::fmt;
use std::u32;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::v4l_sys::V4L2_CID_EXPOSURE_AUTO;
use v4l::video::Capture;
use v4l::{Control, Device, FourCC};

use yuv::YuvPackedImage;

#[derive(Clone)]
pub struct YuvChroma {
    u: u8,
    v: u8,
}

impl YuvChroma {
    pub fn new(u: u8, v: u8) -> Self {
        Self { u: u, v: v }
    }

    fn distance(&self, other: &YuvChroma) -> u32 {
        // Ew
        let dx = i32::from(self.u) - i32::from(other.u);
        let dy = i32::from(self.v) - i32::from(other.v);

        // println!("dx: {}, dy:{}", dx, dy);

        // Powers of two are always positive
        u32::try_from(dx.pow(2) + dy.pow(2)).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ClosestColor {
    Red,
    Green,
    Blue,
    None,
}

impl ClosestColor {
    fn values() -> [Self; 4] {
        [Self::Red, Self::Green, Self::Blue, Self::None]
    }

    fn chroma(&self) -> YuvChroma {
        match self {
            // ClosestColor::Blue => YuvChroma::new(127, -64),
            // ClosestColor::Red => YuvChroma::new(-32, 96),
            // ClosestColor::Green => YuvChroma::new(-64, -96),
            // ClosestColor::None => YuvChroma::new(0, 0),
            ClosestColor::Blue => YuvChroma::new(167, 47),
            ClosestColor::Red => YuvChroma::new(113, 150),
            ClosestColor::Green => YuvChroma::new(106, 99),
            ClosestColor::None => YuvChroma::new(128, 128),
        }
    }

    fn closest(y: u8, u: u8, v: u8) -> ClosestColor {
        let input: YuvChroma = YuvChroma::new(u, v);

        // Clip darker colors
        if y < 128 {
            return ClosestColor::None
        }

        let mut closest_color = ClosestColor::None;
        let mut min = u32::MAX;
        for value in ClosestColor::values() {
            let distance = value.chroma().distance(&input);
            if distance < min {
                min = distance;
                closest_color = value;
            }
        }

        closest_color
    }
}

impl fmt::Display for ClosestColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClosestColor::Red => write!(f, "Red"),
            ClosestColor::Green => write!(f, "Green"),
            ClosestColor::Blue => write!(f, "Blue"),
            ClosestColor::None => write!(f, "None"),
        }
    }
}

pub struct FrameInfo {
    pub colors: Vec<ClosestColor>,
    pub frame_stride: u32,

    pub pixel1_chroma: YuvChroma,

    pub reds: usize,
    pub greens: usize,
    pub blues: usize,
    pub nones: usize,

    pub average: YuvChroma,
}

impl FrameInfo {
    fn new() -> Self {
        Self {
            colors: Vec::new(),
            frame_stride: 0,
            pixel1_chroma: YuvChroma { u: 0, v: 0 },

            reds: 0,
            greens: 0,
            blues: 0,
            nones: 0,

            average: YuvChroma { u: 0, v: 0 }
        }
    }

    fn count(&self, color: ClosestColor) -> usize {
        let mut count: usize = 0;
        for c in self.colors.iter() {
            if color == *c {
                count += 1;
            }
        }

        count
    }

    pub fn closest_color(&self) -> ClosestColor {
        let mut largest = 0;
        let mut largest_count = 0;
        for (index, color_count) in [self.reds, self.greens, self.blues, self.nones].iter().enumerate() {
            if *color_count > largest_count {
                largest = index;
                largest_count = *color_count
            }
        };

        match largest {
            0 => ClosestColor::Red,
            1 => ClosestColor::Green,
            2 => ClosestColor::Blue,
            _ => ClosestColor::None
        }
    }

    pub fn color_coordinate(&self) -> (u32, u32) {
        let mut total = (0, 0);
        for (index, color) in self.colors.iter().enumerate() {
            let x = (index as u32) / self.frame_stride;
            let y = (index as u32) % self.frame_stride;

            if *color == self.closest_color() {
            total = (total.0 + x, total.1 + y);
            }
        }

        total
    }
}

fn get_info<'a>(image: YuvPackedImage<'a, u8>) -> FrameInfo {

    let mut info = FrameInfo::new();
    info.frame_stride = image.yuy_stride;
    info.pixel1_chroma = YuvChroma { u: image.yuy[1], v: image.yuy[3] };

    let mut total: (usize, usize) = (0, 0);
    for a in image.yuy.chunks(4) {
        total = (total.0 + usize::from(a[1]), total.1 + usize::from(a[3]));

        // Y, U, V are interleaved as Y0 U Y1 V
        info.colors.push(ClosestColor::closest(a[0], a[1], a[3]));
        info.colors.push(ClosestColor::closest(a[2], a[1], a[3]));
    }

    info.average = YuvChroma::new(
        u8::try_from(total.0 / (info.colors.len() / 2)).unwrap(),
        u8::try_from(total.1 / (info.colors.len() / 2)).unwrap(),
    );

    for c in info.colors.iter() {
        match c {
            ClosestColor::Red => {
                info.reds += 1
            }
            ClosestColor::Green => {
                info.greens += 1
            }
            ClosestColor::Blue => {
                info.blues += 1
            }
            ClosestColor::None => {
                info.nones += 1
            }
        }
    }

    info    
}

fn print_frame_info(info: &FrameInfo) {
    println!(
        "
        \x1B[2J\x1B[1;1H{} red pixels ({:.3}%), {} green pixels ({:.3}%), {} blue pixels ({:.3}%), and {} uncolored pixels ({:.3}%),
        \nThe average is ({}, {})",
        info.reds,   (info.reds as f32   / info.colors.len() as f32) * 100f32,
        info.greens, (info.greens as f32 / info.colors.len() as f32) * 100f32,
        info.blues,  (info.blues as f32  / info.colors.len() as f32) * 100f32,
        info.nones,  (info.nones as f32  / info.colors.len() as f32) * 100f32,
        
        info.average.u, info.average.v
    );

    println!("The first pixel has a chroma of ({}, {})", info.pixel1_chroma.u, info.pixel1_chroma.v);
}
pub struct CameraVideoStream<'stream> {
    device: Device,
    stream: Stream<'stream>,
}

impl<'stream> CameraVideoStream<'stream> {
    pub fn new() -> std::io::Result<Self> {
        let mut d = Device::new(0)?;

        let mut fmt = d.format()?;
        fmt.width = 1280;
        fmt.height = 720;
        fmt.fourcc = FourCC::new(b"YUYV");
        println!("Format in use:\n{}", d.set_format(&fmt)?);

        match d.set_control(Control {
            id: V4L2_CID_EXPOSURE_AUTO,
            value: v4l::control::Value::Integer(3), // V4L2_EXPOSURE_MANUAL
        }) {
            Err(e) => {
                println!("Failed to set exposure to manual: {}", e);
            }
            _ => (),
        }

        let s = Stream::with_buffers(&mut d, Type::VideoCapture, 4)?;

        Ok(CameraVideoStream {
            device: d,
            stream: s,
        })
    }

    pub fn get_next_frame_info(&mut self) -> FrameInfo {
        let (buf, meta) = self.stream.next().unwrap();

        let a = YuvPackedImage {
            yuy: buf,
            yuy_stride: 2560,
            width: 1280,
            height: 720,
        };

        get_info(a)
    }
}
