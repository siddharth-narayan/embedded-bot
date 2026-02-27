use std::{thread::sleep, time::Duration};

use crate::{camera::CameraVideoStream, control::{Direction, Robot}};

mod control;
mod camera;

fn main() {

    let mut robot = match Robot::new() {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to initialize the robot: {}", e);
            return;
        }
    };

    robot.move_direction(Direction::Forward, Duration::from_millis(500));

    return;

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
}
