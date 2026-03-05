use crate::control::Robot;

impl Robot {
    pub fn startup_action(&mut self) {
        println!("Executing startup action");
    }

    pub fn red_action(&mut self) {
        println!("Executing red action")
    }

    pub fn green_action(&mut self, coordinate: (u32, u32)) {
        println!("Executing green action with coordinate ({}, {})", coordinate.0, coordinate.1)
    }

    pub fn blue_action(&mut self) {
        println!("Executing blue action")
    }
}
