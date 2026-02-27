use std::{thread::sleep, time::Duration};

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use crate::{camera::{CameraVideoStream}, control::{ControlError, forward}};

mod control;
mod camera;

const CONTROLLER_ADDRESS: u16 = 0x2B;

fn main() {
    // let mut device = match LinuxI2CDevice::new("/dev/i2c-1", CONTROLLER_ADDRESS) {
    //     Ok(d) => d,
    //     Err(e) => {
    //         println!("Failed to get the i2c device: {}", e);
    //         return;
    //     }
    // };

    let mut camera_stream = match CameraVideoStream::new() {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to get the camera stream: {}", e);
            return;
        }
    };

    loop {
        camera_stream.get_next_frame_closest_color();
        sleep(Duration::from_millis(500));
    }

    // match begin(&mut device) {
    //     Ok(_) => (),
    //     Err(e) => {
    //         println!("Failed to complete process: {}", e);
    //     }
    // }

}

fn begin(device: &mut LinuxI2CDevice) -> ControlError<LinuxI2CError> {
    
    forward(device)?;

    Ok(())
}


