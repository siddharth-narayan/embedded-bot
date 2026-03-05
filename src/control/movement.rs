use std::{thread::sleep, time::Duration};

use i2cdev::linux::LinuxI2CError;

use crate::control::{Register, Robot, ControlError};

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum Motor {
    ForwardLeft = 0,
    BackwardLeft = 1,
    ForwardRight = 2,
    BackwardRight = 3,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum MotorDirection {
    Forward = 0,
    Reverse = 1,
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

impl Robot {
    pub(super) fn move_motor(
        &mut self,
        motor: Motor,
        direction: MotorDirection,
        speed: u8,
    ) -> ControlError<LinuxI2CError> {
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
            &[
                Motor::ForwardRight as u8,
                MotorDirection::Forward as u8,
                0u8,
            ],
        )?;

        self.write_block_data(
            Register::MotorControl as u8,
            &[
                Motor::BackwardLeft as u8,
                MotorDirection::Forward as u8,
                0u8,
            ],
        )?;

        self.write_block_data(
            Register::MotorControl as u8,
            &[
                Motor::BackwardRight as u8,
                MotorDirection::Forward as u8,
                0u8,
            ],
        )?;

        Ok(())
    }

    pub fn move_rotate(
        &mut self,
        direction: Rotation,
        speed: u8,
        duration: Duration,
    ) -> ControlError<LinuxI2CError> {
        match direction {
            Rotation::CounterClockwise => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Forward, speed)?;
            }

            Rotation::Clockwise => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Reverse, speed)?;
            }
        }

        sleep(duration);

        self.stop()?;

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
                self.move_motor(Motor::ForwardLeft, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Reverse, speed)?;
            }

            Direction::Right => {
                self.move_motor(Motor::ForwardLeft, MotorDirection::Forward, speed)?;
                self.move_motor(Motor::ForwardRight, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::BackwardLeft, MotorDirection::Reverse, speed)?;
                self.move_motor(Motor::BackwardRight, MotorDirection::Forward, speed)?;
            }
        }

        sleep(duration);

        self.stop()?;

        Ok(())
    }
}