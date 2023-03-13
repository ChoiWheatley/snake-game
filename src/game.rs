use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rect};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.8, 0., 0., 1.];
const BORDER_COLOR: Color = [0., 0., 0., 1.];
const GAMEOVER_COLOR: Color = [0.9, 0., 0., 0.5];
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.;

pub struct Game {
    snake: Snake,

    food_exists: bool,

    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

trait IsInside {
    fn is_inside(&self, start: (i32, i32), width: i32, height: i32) -> bool;
}

impl IsInside for (i32, i32) {
    fn is_inside(&self, start: (i32, i32), width: i32, height: i32) -> bool {
        let max_width = start.0 + width;
        let max_height = start.1 + height;
        start.0 < self.0 && self.0 < max_width && start.1 < self.1 && self.1 < max_height
    }
}

/// private
impl Game {
    fn fail_condition(&self) -> bool {
        let next = self.snake.next_head_position();
        self.snake.overlap_with(next.0, next.1)
            || !self
                .snake
                .head_position()
                .is_inside((0, 0), self.width - 1, self.height - 1)
    }
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Game {
            snake: Snake::new(
                rand::thread_rng().gen_range(1..width),
                rand::thread_rng().gen_range(1..height),
            ),
            food_exists: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                // can't move direction
                return;
            }
            self.update_snake(Some(d));
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.for_each(|x, y| {
            draw_rect(SNAKE_COLOR, x, y, 1, 1, &con, g);
        });

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, &con, g);
        }

        // draw border
        draw_rect(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rect(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rect(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rect(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rect(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (x, y) = self.snake.head_position();
        if self.food_exists && self.food_x == x && self.food_y == y {
            self.food_exists = false;
            self.snake.grow_tail();
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut x = rng.gen_range(1..self.width - 1);
        let mut y = rng.gen_range(1..self.width - 1);
        while self.snake.overlap_with(x, y) {
            x = rng.gen_range(1..self.width - 1);
            y = rng.gen_range(1..self.width - 1);
        }

        self.food_x = x;
        self.food_y = y;
        self.food_exists = true;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(
            thread_rng().gen_range(1..self.width - 1),
            thread_rng().gen_range(1..self.height - 1),
        );
        self.waiting_time = 0.;
        self.food_exists = false;
        self.game_over = false;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.fail_condition() {
            self.game_over = true;
        }
        self.snake.move_forward(dir);
        self.check_eating();
        self.waiting_time = 0.;
    }
}
