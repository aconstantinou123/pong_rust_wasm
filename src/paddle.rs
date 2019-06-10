use wasm_bindgen::prelude::*;
use crate::ball::Ball;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum PaddleType {
    Computer,
    Player,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Paddle {
    x: i32,
    y: i32,
    height: i32,
    width: i32,
    speed_y: i32,
    y_positions: Vec<i32>,
    paddle_type: PaddleType,
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
        let paddle_type = PaddleType::Player;
        Paddle {
            x,
            y,
            height,
            width,
            speed_y,
            y_positions,
            paddle_type,
        }
    }

    pub fn new_computer() -> Paddle {
        let x = 1130;
        let y = 300;
        let height = 100;
        let width = 20;
        let speed_y = 20;
        let y_positions = (y.. (y + height)).collect();
        let paddle_type = PaddleType::Computer;
        Paddle {
            x,
            y,
            height,
            width,
            speed_y,
            y_positions,
            paddle_type,
        }
    }

    pub fn get_paddle_type(&self) -> PaddleType {
        self.paddle_type.clone()
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


    pub fn computer_ai(&mut self, ball: &Ball){
            let mut diff = -((self.y + (self.height / 2)) -  ball.get_y());
            if diff < 0 && diff < -9 {
                diff = -10;
            } else if diff > 0 && diff > 9  {
                diff = 10;
            }
        if ball.get_x() > 300 && ball.get_y() >= self.y && !self.get_y_positions().contains(&ball.get_y()) {
            self.move_up(diff)
        } else if ball.get_x() > 300 && ball.get_y() <= self.y && !self.get_y_positions().contains(&ball.get_y()) {
            self.move_down(diff)
        }
    }

    pub fn move_up(&mut self, speed: i32) {
        if self.y < 650 {
            self.speed_y = speed;
            self.y += self.speed_y;
        } else {
            self.y = 650;
            self.speed_y = 0;
            self.y_positions = (self.y.. (self.y + self.height)).collect();
        }
        self.y_positions = self.y_positions
            .iter()
            .map(|y| y + self.speed_y)
            .collect();
    }

    pub fn move_down(&mut self, speed: i32) {
        if self.y > 0 {
            self.speed_y = speed;
            self.y += self.speed_y;
        } else {
            self.y = 0;
            self.speed_y = 0;
            self.y_positions = (self.y.. (self.y + self.height)).collect();
        }
        self.y_positions = self.y_positions
            .iter()
            .map(|y| y + self.speed_y)
            .collect();
    }
}
