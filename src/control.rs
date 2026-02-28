use std::{thread::sleep, time::Duration};

use i2cdev::{
    core::I2CDevice,
    linux::{LinuxI2CDevice, LinuxI2CError},
};

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

#[repr(u8)]
#[derive(Clone, Copy)]
enum Motor {
    ForwardLeft = 0,
    ForwardRight = 1,
    BackwardLeft = 2,
    BackwardRight = 3,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum MotorDirection {
    Forward = 1,
    Reverse = 0,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

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

    pub fn test(&mut self) {
        for motor in [Motor::ForwardLeft, Motor::ForwardRight, Motor::BackwardLeft, Motor::BackwardRight] {
            _ = self.move_motor(motor, MotorDirection::Forward, 128);
            sleep(Duration::from_millis(500));
            _ = self.move_motor(motor, MotorDirection::Reverse, 128);
            
            sleep(Duration::from_millis(1500));
        }

        _ = self.stop();

    }

    fn write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), LinuxI2CError> {
        self._internal_device
            .smbus_write_i2c_block_data(register, values)
    }

    fn move_motor(&mut self, motor: Motor, direction: MotorDirection, speed: u8) -> ControlError<LinuxI2CError> {
        self.write_block_data(
            Register::MotorControl as u8,
            &[motor as u8, direction as u8, speed],
        )?;

        Ok(())
    }

    pub fn stop(&mut self) -> ControlError<LinuxI2CError> {
        self.write_block_data(
            Register::MotorControl as u8,
            &[Motor::ForwardLeft as u8, MotorDirection::Forward as u8, 0u8],
        )?;

        self.write_block_data(
            Register::MotorControl as u8,
            &[Motor::ForwardRight as u8, MotorDirection::Forward as u8, 0u8],
        )?;

        self.write_block_data(
            Register::MotorControl as u8,
            &[Motor::BackwardLeft as u8, MotorDirection::Forward as u8, 0u8],
        )?;

        self.write_block_data(
            Register::MotorControl as u8,
            &[Motor::BackwardRight as u8, MotorDirection::Forward as u8, 0u8],
        )?;

        Ok(())
    }
    
    pub fn move_direction(
        &mut self,
        direction: Direction,
        speed: u8,
        duration: Duration,
    ) -> ControlError<LinuxI2CError> {
        match direction {
            Direction::Forward => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Forward, speed)?;
            }

            Direction::Backward => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Reverse, speed)?;
            }

            Direction::Left => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Forward, speed)?;
            }

            Direction::Right => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Forward, speed)?;
            }
        }

        sleep(duration);

        Ok(())
    }
}
