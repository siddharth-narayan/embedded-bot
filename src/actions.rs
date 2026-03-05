use std::time::Duration;

use crate::control::{Robot, movement::Rotation};

impl Robot {
    pub fn startup_action(&mut self) {
        println!("Executing startup action");

        let _ = self.move_rotate(Rotation::Clockwise, 255, Duration::from_millis(500));
    }

    pub fn idle_action(&mut self) {
        // Do nothing :)
    }

    pub fn red_action(&mut self) {
        println!("Executing red action");

        let _ = self.move_rotate(Rotation::Clockwise, 255, Duration::from_millis(250));
    }

    pub fn green_action(&mut self, coordinate: (u32, u32)) {
        println!(
            "Executing green action with coordinate ({}, {})",
            coordinate.0, coordinate.1
        )
    }

    pub fn blue_action(&mut self, coordinate: (u32, u32)) {
        println!("Executing blue action");

        // let direction = coordinate_to_direction();

        let direction = crate::control::movement::Direction::Left;
        self.move_direction(direction, 255, Duration::from_millis(100));
    }
}
