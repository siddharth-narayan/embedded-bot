use core::fmt;
use std::u32;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::v4l_sys::V4L2_CID_EXPOSURE_AUTO;
use v4l::video::Capture;
use v4l::{Control, Device, Format, FourCC};

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
            ClosestColor::Blue => YuvChroma::new(239, 30),
            ClosestColor::Red => YuvChroma::new(60, 210),
            ClosestColor::Green => YuvChroma::new(87, 80),
            ClosestColor::None => YuvChroma::new(128, 128),
        }
    }

    fn closest(y: u8, u: u8, v: u8) -> ClosestColor {
        let input: YuvChroma = YuvChroma::new(u, v);

        // Clip darker colors
        if y < 32 {
            return ClosestColor::None;
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
    fn new<'a>(image: YuvPackedImage<'a, u8>) -> Self {
        let mut colors = Vec::new();
        for a in image.yuy.chunks(4) {
            // Y, U, V are interleaved as Y0 U Y1 V
            colors.push(ClosestColor::closest(a[0], a[1], a[3]));
            colors.push(ClosestColor::closest(a[2], a[1], a[3]));
        }

        Self {
            frame_stride: image.yuy_stride,
            pixel1_chroma: YuvChroma {
                u: image.yuy[1],
                v: image.yuy[3],
            },

            reds: Self::count(&colors, ClosestColor::Red),
            greens: Self::count(&colors, ClosestColor::Green),
            blues: Self::count(&colors, ClosestColor::Blue),
            nones: Self::count(&colors, ClosestColor::None),

            colors: colors,
            average: Self::average(image),
        }
    }

    // Equivalent of ColorCounter
    fn count(colors: &Vec<ClosestColor>, color: ClosestColor) -> usize {
        let mut count: usize = 0;
        for c in colors {
            if color == *c {
                count += 1;
            }
        }

        count
    }

    fn average<'a>(image: YuvPackedImage<'a, u8>) -> YuvChroma {
        let mut total: (usize, usize) = (0, 0);

        for a in image.yuy.chunks(4) {
            total = (total.0 + usize::from(a[1]), total.1 + usize::from(a[3]));
        }

        YuvChroma::new(
            u8::try_from(total.0 / (image.yuy.len() / 4)).unwrap(),
            u8::try_from(total.1 / (image.yuy.len() / 4)).unwrap(),
        )
    }

    pub fn closest_color(&self) -> ClosestColor {
        let mut largest = 0;
        let mut largest_count = 0;
        for (index, color_count) in [self.reds, self.greens, self.blues, self.nones / 33]
            .iter()
            .enumerate()
        {
            if *color_count > largest_count {
                largest = index;
                largest_count = *color_count
            }
        }

        match largest {
            0 => ClosestColor::Red,
            1 => ClosestColor::Green,
            2 => ClosestColor::Blue,
            _ => ClosestColor::None,
        }
    }

    // Equivalent of ColorLocator
    pub fn color_coordinate(&self) -> (u32, u32) {
        let mut total = (0, 0);
        for (index, color) in self.colors.iter().enumerate() {
            let x = (index as u32) / self.frame_stride;
            let y = (index as u32) % self.frame_stride;

            if *color == self.closest_color() {
                total = (total.0 + x, total.1 + y);
            }
        }

        (
            total.0 / self.colors.len() as u32,
            total.1 / self.colors.len() as u32,
        )
    }

    pub fn print(&self) {
        println!(
        "
        {} red pixels ({:.3}%), {} green pixels ({:.3}%), {} blue pixels ({:.3}%), and {} uncolored pixels ({:.3}%),
        \nThe average is ({}, {})",
        self.reds,   (self.reds as f32   / self.colors.len() as f32) * 100f32,
        self.greens, (self.greens as f32 / self.colors.len() as f32) * 100f32,
        self.blues,  (self.blues as f32  / self.colors.len() as f32) * 100f32,
        self.nones,  (self.nones as f32  / self.colors.len() as f32) * 100f32,

        self.average.u, self.average.v
    );

        println!(
            "The first pixel has a chroma of ({}, {})",
            self.pixel1_chroma.u, self.pixel1_chroma.v
        );
    }
}

pub struct CameraVideoStream<'stream> {
    _device: Device,
    stream: Stream<'stream>,
    format: Format,
}

impl<'stream> CameraVideoStream<'stream> {
    pub fn new() -> std::io::Result<Self> {
        let mut d = Device::new(0)?;

        let fmt = Format::new(1920, 1080, FourCC::new(b"YUYV"));
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
            _device: d,
            format: fmt,
            stream: s,
        })
    }

    pub fn get_next_frame_info(&mut self) -> FrameInfo {
        let (buf, _meta) = self.stream.next().unwrap();

        let image = YuvPackedImage {
            yuy: buf,
            yuy_stride: self.format.stride,
            width: self.format.width,
            height: self.format.height,
        };

        FrameInfo::new(image)
    }
}
