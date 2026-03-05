use i2cdev::linux::LinuxI2CError;

use crate::control::{Register, Robot, ControlError};

#[derive(Clone)]
pub struct LightColor {
    r: u8,
    g: u8,
    b: u8,
}

impl LightColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        LightColor {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl Robot {
    pub fn set_light(
        &mut self,
        light: u8,
        color: LightColor,
    ) -> ControlError<LinuxI2CError> {

        self.write_block_data(
            Register::WQ2812BrightnessAlone as u8,
            &[light, color.r, color.g, color.b],
        )?;

        Ok(())
    }
}