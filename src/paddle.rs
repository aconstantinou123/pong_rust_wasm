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
pub struct Paddle {
    x: i32,
    y: i32,
    height: i32,
    width: i32,
    speed_y: i32,
    y_positions: Vec<i32>
}

#[wasm_bindgen]
impl Paddle {

    pub fn new_player() -> Paddle {
        let x = 100;
        let y = 300;
        let height = 100;
        let width = 20;
        let speed_y = 20;
        let y_positions = (y.. (y + height)).collect();
        Paddle {
            x,
            y,
            height,
            width,
            speed_y,
            y_positions,
        }
    }

    pub fn get_y_positions(&self) -> Vec<i32> {
        self.y_positions.clone()
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_speed_y(&self) -> i32 {
        self.speed_y
    }

    pub fn move_up(&mut self) {
        self.y += self.speed_y
    }

    pub fn move_down(&mut self) {
        self.y -= self.speed_y
    }
}
