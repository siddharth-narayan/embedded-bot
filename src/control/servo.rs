use std::{thread::sleep, time::Duration};

use std::cmp::min;

use i2cdev::linux::LinuxI2CError;

use crate::control::{ControlError, Register, Robot};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Servo {
    CameraPan = 1,
    CameraTilt = 2,
}

impl Robot {
    pub fn move_servo(&mut self, servo: Servo, mut angle: u8) -> ControlError<LinuxI2CError> {
        if angle > 180 {
            angle = 180
        }

        if servo == Servo::CameraTilt {
            angle = min(angle, 100);
        }

        self.write_block_data(Register::ServoControl, &[servo as u8, angle])?;

        Ok(())
    }

    pub(super) fn test_servos(&mut self) -> ControlError<LinuxI2CError> {
        for x in 0u8..255 {
            self.move_servo(Servo::CameraPan, x)?;
            sleep(Duration::from_millis(10));
        }

        // self.move_servo(Servo::CameraTilt, 0)?;
        for x in 10u8..100 {
            self.move_servo(Servo::CameraTilt, x)?;
            sleep(Duration::from_millis(10));
        }

        self.move_servo(Servo::CameraPan, 90)?;
        self.move_servo(Servo::CameraTilt, 50)?;
        Ok(())
    }
}
