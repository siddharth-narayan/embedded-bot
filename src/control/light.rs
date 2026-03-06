use std::{thread::sleep, time::Duration};

use i2cdev::linux::LinuxI2CError;

use crate::control::{ControlError, Register, Robot};

#[derive(Clone, Copy)]
pub struct LightColor {
    r: u8,
    g: u8,
    b: u8,
}

impl LightColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        LightColor { r: r, g: g, b: b }
    }
}

impl Robot {
    pub fn set_light(&mut self, light: u8, color: LightColor) -> ControlError<LinuxI2CError> {
        if light > 8 {
            return Ok(());
        }

        self.write_block_data(
            Register::WQ2812BrightnessAlone,
            &[light, color.r, color.g, color.b],
        )?;

        Ok(())
    }

    pub fn set_all_lights(&mut self, color: LightColor) -> ControlError<LinuxI2CError> {
        for l in 0u8..9 {
            self.set_light(l, color)?;
        }

        Ok(())
    }

    pub(super) fn test_lights(&mut self) -> ControlError<LinuxI2CError> {
        for _ in 0..3 {
            self.set_all_lights(LightColor::new(255, 0, 0))?;
            sleep(Duration::from_millis(1000));

            self.set_all_lights(LightColor::new(0, 255, 0))?;
            sleep(Duration::from_millis(1000));

            self.set_all_lights(LightColor::new(0, 0, 255))?;
            sleep(Duration::from_millis(1000));
        }

        Ok(())
    }
}
