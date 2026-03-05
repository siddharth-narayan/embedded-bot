use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::{
    camera::{CameraVideoStream, ClosestColor},
    control::{Robot},
};

mod actions;
mod camera;
mod control;

fn main() {
    let mut robot = match Robot::new() {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to initialize the robot: {}", e);
            return;
        }
    };

    _ = robot.test();

    let mut camera_stream = match CameraVideoStream::new() {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to get the camera stream: {}", e);
            return;
        }
    };

    // Actions

    robot.startup_action();

    let mut last_action_time = SystemTime::UNIX_EPOCH;
    loop {
        let info = camera_stream.get_next_frame_info();
        let closest_color = info.closest_color();

        let time_since_last_action = last_action_time.elapsed().unwrap();

        match closest_color {
            ClosestColor::Red => {
                if time_since_last_action > Duration::from_millis(2000) {
                    last_action_time = SystemTime::now();
                    robot.red_action()
                }
            }

            ClosestColor::Green => {
                if time_since_last_action > Duration::from_millis(50) {
                    last_action_time = SystemTime::now();
                    robot.green_action(info.color_coordinate())
                }
            }

            ClosestColor::Blue => {
                if time_since_last_action > Duration::from_millis(2000) {
                    last_action_time = SystemTime::now();
                    robot.blue_action()
                }
            }

            ClosestColor::None => (),
        }
        sleep(Duration::from_millis(16));
    }
}
