use std::{
    process::exit,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::control::{Robot, light::LightColor, movement::Rotation};

pub fn timer_check(start_time: SystemTime) {
    if let Ok(duration) = start_time.elapsed() {
        if duration > Duration::from_secs(45) {
            println!("Program has continued for more than 45 seconds, exiting");
            exit(0)
        }
    }
}

impl Robot {
    pub fn startup_action(&mut self) {
        println!("Executing startup action");

        let _ = self.move_rotate(Rotation::Clockwise, 255, Duration::from_millis(1000));
    }

    pub fn idle_action(&mut self) {
        // Do nothing :)
    }

    pub fn red_action(&mut self) {
        println!("Executing red action");

        let _ = self.move_rotate(Rotation::Clockwise, 255, Duration::from_millis(500));
    }

    pub fn green_action(&mut self, coordinate: (usize, usize)) {
        println!(
            "Executing green action with coordinate ({}, {})",
            coordinate.0, coordinate.1
        );

        _ = self.set_all_lights(LightColor::new(0, 255, 0));
        sleep(Duration::from_millis(250));
        _ = self.set_all_lights(LightColor::new(0, 0, 0));
    }

    pub fn blue_action(&mut self, coordinate: (usize, usize)) {
        println!(
            "Executing blue action with coordinate ({}, {})",
            coordinate.0, coordinate.1
        );

        // let direction = coordinate_to_direction();

        let direction = crate::control::movement::Direction::Left;
        _ = self.move_direction(direction, 255, Duration::from_millis(100));
    }
}
