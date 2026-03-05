use i2cdev::linux::LinuxI2CError;

use crate::control::{Register, Robot, ControlError};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Servo {
    CameraPan = 0,
    CameraTilt = 1,
}

impl Robot {
    pub fn move_servo(
        &mut self,
        servo: Servo,
        mut angle: u8,
    ) -> ControlError<LinuxI2CError> {

        if angle > 180 {
            angle = 180
        }

        // if servo == Servo::2 {
        //   angle = max(angle, 100);
        // }

        self.write_block_data(
            Register::ServoControl as u8,
            &[servo as u8, angle],
        )?;

        Ok(())
    }
}