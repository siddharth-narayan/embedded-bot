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
        self.move_servo(Servo::CameraPan, 70)?;
        self.move_servo(Servo::CameraTilt, 70)?;

        for light in 0u8..8 {
            self.set_light(light, LightColor::new(255, 0, 0))?;
            sleep(Duration::from_millis(100))
        }

        let test_speed = 255u8;
        for motor in [
            Motor::ForwardLeft,
            Motor::ForwardRight,
            Motor::BackwardLeft,
            Motor::BackwardRight,
        ] {
            self.move_motor(motor, MotorDirection::Forward, test_speed)?;
            sleep(Duration::from_millis(500));

            self.move_motor(motor, MotorDirection::Reverse, test_speed)?;
            sleep(Duration::from_millis(500));

            self.stop()?;
            sleep(Duration::from_millis(1000));
        }

        for direction in [
            Direction::Forward,
            Direction::Backward,
            Direction::Left,
            Direction::Right,
        ] {
            self.move_direction(direction, test_speed, Duration::from_millis(2000))?;
        }

        for rotation in [Rotation::Clockwise, Rotation::CounterClockwise] {
            self.move_rotate(rotation, test_speed, Duration::from_millis(2000))?;
        }

        self.stop()?;

        Ok(())
    }

    fn write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), LinuxI2CError> {
        self._internal_device
            .smbus_write_i2c_block_data(register, values)
    }
}
