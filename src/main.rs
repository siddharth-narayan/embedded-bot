use std::time::{Duration, SystemTime};

use crate::{
    camera::{CameraVideoStream, ClosestColor},
    control::Robot,
};

use crate::control::light::LightColor;

mod actions;
mod camera;
mod control;

fn main() {
    let test = std::env::args().any(|a| a == "--test");
    let debug = std::env::args().any(|a| a == "--debug");

    let mut robot = match Robot::new() {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to initialize the robot: {}", e);
            return;
        }
    };

    if test {
        _ = robot.test();
        std::process::exit(0)
    }

    let mut camera_stream = match CameraVideoStream::new() {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to get the camera stream: {}", e);
            return;
        }
    };

    // Actions

    robot.startup_action();

    let start_time = SystemTime::now();
    let mut last_action_time = SystemTime::UNIX_EPOCH;
    loop {
        robot.timer_check(start_time);
        let frame = camera_stream.get_next_frame();

        let closest_color = frame.closest_color();

        let time_since_last_action = last_action_time.elapsed().unwrap();

        match closest_color {
            ClosestColor::Red => {
                frame.print();

                _ = robot.set_all_lights(LightColor::red());

                if time_since_last_action > Duration::from_millis(2000) {
                    last_action_time = SystemTime::now();
                    robot.red_action()
                }
            }

            ClosestColor::Green => {
                frame.print();

                _ = robot.set_all_lights(LightColor::green());

                if time_since_last_action > Duration::from_millis(50) {
                    last_action_time = SystemTime::now();
                    robot.green_action(frame.color_coordinate(), frame.dimensions())
                }
            }

            ClosestColor::Blue => {
                frame.print();

                _ = robot.set_all_lights(LightColor::blue());

                if time_since_last_action > Duration::from_millis(50) {
                    last_action_time = SystemTime::now();
                    robot.blue_action(frame.color_coordinate(), frame.dimensions())
                }
            }

            ClosestColor::None => {
                if debug {
                    frame.print();
                }

                _ = robot.set_all_lights(LightColor::white());

                robot.idle_action()
            }
        }
    }
}
