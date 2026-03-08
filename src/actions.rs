use std::{
    process::exit,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::control::{Robot, light::LightColor, movement::{Direction, Rotation}};

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

    pub fn green_action(&mut self, coordinate: (usize, usize), dimensions: (usize, usize)) {

        // The camera seems to be flipped, so use the Y axis as horizontal direction
        let direction = if coordinate.1 < (dimensions.1 / 2) {
            Rotation::CounterClockwise
        } else {
            Rotation::Clockwise
        };

        println!(
            "Executing blue action -- coordinate: ({}, {}), direction: {}",
            coordinate.0, coordinate.1, direction
        );

        _ = self.move_rotate(direction, 60, Duration::from_millis(250));
        
    }

    pub fn blue_action(&mut self, coordinate: (usize, usize), dimensions: (usize, usize)) {

        // The camera seems to be flipped, so use the Y axis as horizontal direction
        let direction = if coordinate.1 < (dimensions.1 / 2) {
            Direction::Left
        } else {
            Direction::Right
        };

        println!(
            "Executing blue action -- coordinate: ({}, {}), direction: {}",
            coordinate.0, coordinate.1, direction
        );

        _ = self.move_direction(direction, 60, Duration::from_millis(250));
    }
}
