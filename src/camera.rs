use core::fmt;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::v4l_sys::{
    V4L2_CID_EXPOSURE, V4L2_CID_EXPOSURE_ABSOLUTE, V4L2_CID_EXPOSURE_AUTO, V4L2_CID_IRIS_ABSOLUTE,
};
use v4l::video::Capture;
use v4l::{Control, Device, FourCC};

use yuv::{YuvPackedImage, YuvRange};

const COLOR_RED: Color = Color { r: 255, g: 0, b: 0 };

const COLOR_GREEN: Color = Color { r: 0, g: 255, b: 0 };

const COLOR_BLUE: Color = Color { r: 0, g: 0, b: 255 };

const COLOR_NONE: Color = Color {
    r: 16,
    g: 16,
    b: 16,
};

pub enum ClosestColor {
    Red,
    Green,
    Blue,
    None,
}

impl ClosestColor {
    pub fn closest(c: &Color) -> Self {
        let all_colors = [COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_NONE];

        let mut min_color_idx = 0;
        let mut min_difference = u32::MAX;

        for (index, color) in all_colors.iter().enumerate() {
            let difference = c.abs_diff(&color);

            println!("Difference between {} and {} is {}", c, color, difference);

            if difference < min_difference {
                min_color_idx = index;
                min_difference = difference;
            }
        }

        match min_color_idx {
            0 => ClosestColor::Red,
            1 => ClosestColor::Green,
            2 => ClosestColor::Blue,
            _ => ClosestColor::None,
        }
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

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {}, g: {}, b: {}", self.r, self.g, self.b)
    }
}

impl Color {
    fn abs_diff(&self, rhs: &Self) -> u32 {
        let diff = (self.r.abs_diff(rhs.r) as u32).pow(2)
            + (self.g.abs_diff(rhs.g) as u32).pow(2)
            + (self.b.abs_diff(rhs.b) as u32).pow(2);

        diff
    }
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

        // match d.set_control(Control {
        //     id: V4L2_CID_EXPOSURE_ABSOLUTE,
        //     value: v4l::control::Value::Integer(625) // Where "1" would be 1/10000 of a second exposure time
        // }) {
        //     Err(e) => {
        //         println!("Failed to set exposure time: {}", e);
        //     }
        //     _ => ()
        // }

        // match d.set_control(Control {
        //     id: V4L2_CID_IRIS_ABSOLUTE,
        //     value: v4l::control::Value::Integer(10) // Where "1" would be 1/10000 of a second exposure time
        // }) {
        //     Err(e) => {
        //         println!("Failed to set aperture to wide open: {}", e);
        //     }
        //     _ => ()
        // }

        let s = Stream::with_buffers(&mut d, Type::VideoCapture, 4)?;

        Ok(CameraVideoStream {
            device: d,
            stream: s,
        })
    }

    fn average_next_frame<'a>(&mut self) -> Color {
        let (buf, meta) = self.stream.next().unwrap();
        println!(
            "Buffer size: {}, seq: {}, timestamp: {}",
            buf.len(),
            meta.sequence,
            meta.timestamp,
        );

        let a = YuvPackedImage {
            yuy: buf,
            yuy_stride: 2560,
            width: 1280,
            height: 720,
        };

        let mut out = [0; 2764800];
        yuv::yuyv422_to_rgb(
            &a,
            &mut out,
            3840,
            YuvRange::Full,
            yuv::YuvStandardMatrix::Bt709,
        )
        .unwrap();

        // Average the frames
        let mut r: u64 = 0;
        let mut g: u64 = 0;
        let mut b: u64 = 0;

        for (index, size) in out.iter().enumerate() {
            match index % 3 {
                0 => r += *size as u64,
                1 => g += *size as u64,
                2 => b += *size as u64,
                _ => (),
            }
        }

        // Can be unwrapped because the average will never be > u8::MAX
        let channel_pixels = out.len() as u64 / 3;
        Color {
            r: (r / channel_pixels).try_into().unwrap(),
            g: (g / channel_pixels).try_into().unwrap(),
            b: (b / channel_pixels).try_into().unwrap(),
        }
    }

    pub fn get_next_frame_closest_color(&mut self) {
        let average_color = self.average_next_frame();

        println!(
            "The closest color to {} is {}",
            &average_color,
            ClosestColor::closest(&average_color)
        )
    }
}
