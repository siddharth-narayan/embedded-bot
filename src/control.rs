use std::{thread::sleep, time::Duration};

use i2cdev::{core::I2CDevice, linux::{LinuxI2CDevice, LinuxI2CError}};

pub type ControlError<E> = Result<(), E>;
#[repr(u8)]
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
enum Motor {
    ForwardLeft = 0,
    ForwardRight = 1,
    BackwardLeft = 2,
    BackwardRight = 3,
}

#[repr(u8)]
enum MotorDirection {
    Forward = 1,
    Reverse = 0,
}

pub fn forward(device: &mut LinuxI2CDevice) -> ControlError<LinuxI2CError> {
    for motor in [Motor::ForwardLeft, Motor::ForwardRight, Motor::BackwardLeft, Motor::BackwardRight] {
        device.smbus_write_block_data(
            Register::MotorControl as u8, &[motor as u8, MotorDirection::Forward as u8, 255u8],
        )?;

        sleep(Duration::from_millis(10));
    }

    Ok(())
}