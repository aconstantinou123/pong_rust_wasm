use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Ball {
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32,
    radius: i32,
    in_play: bool,
}

#[wasm_bindgen]
impl Ball {
    pub fn new(x: i32, y: i32, speed_x: i32, speed_y: i32) -> Ball {
        Ball {
            x,
            y,
            speed_x,
            speed_y,
            in_play: true,
            radius: 10,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

     pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_speed_x(&self) -> i32 {
        self.speed_x
    }

    pub fn get_speed_y(&self) -> i32 {
        self.speed_y
    }

    pub fn get_radius(&self) -> i32 {
        self.radius
    }

    pub fn get_in_play(&self) -> bool {
        self.in_play
    }

    pub fn move_ball(&mut self) {
        //bottom
        log!("{}", self.y - self.radius);
        if self.y - self.radius <= 0 {
            self.speed_y = -self.speed_y;
        }
        if self.y + self.radius >= 750 {
            self.speed_y = -self.speed_y;
        }
        if self.x - self.radius <= 0 {
            self.speed_x = -self.speed_x;
        } 
        if self.x + self.radius >= 1250 {
            self.speed_x = -self.speed_x;
        } 
        self.x += self.speed_x;
        self.y += self.speed_y;
    }
    
} 
