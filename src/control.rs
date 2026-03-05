use std::{thread::sleep, time::Duration};

use i2cdev::{
    core::I2CDevice,
    linux::{LinuxI2CDevice, LinuxI2CError},
};

use crate::control::{light::LightColor, movement::{Direction, Motor, MotorDirection, Rotation}, servo::Servo};

pub mod light;
pub mod servo;
pub mod movement;

pub type ControlError<E> = Result<(), E>;

const CONTROLLER_ADDRESS: u16 = 0x2B;

#[repr(u8)]
#[derive(Clone, Copy)]
enum Register {
    MotorControl = 0x01,
    ServoControl = 0x02,
    WQ2812All = 0x03,
    WQ2812Alone = 0x04,
    IRSwitch = 0x05,
    BeepSwitch = 0x06,
    UltrasonicSwitch = 0x07,
    WQ2812BrightnessAll = 0x08,
    WQ2812BrightnessAlone = 0x09,
}

// 0 - 3

pub struct Robot {
    _internal_device: LinuxI2CDevice,
}

impl Robot {
    pub fn new() -> Result<Self, LinuxI2CError> {
        let device = LinuxI2CDevice::new("/dev/i2c-1", CONTROLLER_ADDRESS)?;

        Ok(Robot {
            _internal_device: device,
        })
    }

    pub fn test(&mut self) -> ControlError<LinuxI2CError> {
        // self.test_movement()?;
        self.test_servos()?;
        self.test_lights()?;

        Ok(())
    }

    fn write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), LinuxI2CError> {
        self._internal_device
            .smbus_write_i2c_block_data(register, values)
    }
}
