#![allow(dead_code)]

use std::{thread::sleep, time::Duration};

use i2cdev::linux::LinuxI2CError;

use crate::control::{ControlError, Register, Robot};

#[derive(Clone, Copy)]
pub struct LightColor {
    r: u8,
    g: u8,
    b: u8,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LightStatus {
    On = 0,
    Off = 1,
}

impl LightColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        LightColor { r: r, g: g, b: b }
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}

impl Robot {
    pub fn set_light_on_off(&mut self, light: u8, status: LightStatus) -> ControlError<LinuxI2CError> {
        if light > 8 {
            return Ok(());
        }

        self.write_block_data(
            Register::WQ2812Alone,
            &[light, status as u8, 0],
        )?;

        Ok(())
    }


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
            sleep(Duration::from_millis(250));

            self.set_all_lights(LightColor::new(0, 255, 0))?;
            sleep(Duration::from_millis(250));

            self.set_all_lights(LightColor::new(0, 0, 255))?;
            sleep(Duration::from_millis(250));
        }

        Ok(())
    }
}
