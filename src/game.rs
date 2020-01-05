use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Block, Direction, Snake};

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    blocks: HashSet<Block>,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2, width - 1, height - 1),
            blocks: HashSet::new(),
            waiting_time: 0.0,
            food_exists: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            game_over: false,
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

        match dir {
            Some(d) => {
                if d == self.snake.head_direction().opposite() {
                    return;
                }
            }
            None => return,
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        for b in &self.blocks {
            draw_block(BORDER_COLOR, b.x, b.y, con, g);
        }

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
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
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);
        let overlap = self.snake.overlap_tail(next_x, next_y);
        let crash = self.check_in_blocks(next_x, next_y);

        !overlap && !crash
    }

    fn check_in_blocks(&self, x: i32, y: i32) -> bool {
        let b = Block { x, y };
        self.blocks.contains(&b)
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) || self.check_in_blocks(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2, self.width - 1, self.height - 1);
        self.waiting_time = 0.0;
        self.food_exists = false;
        self.game_over = false;
    }

    pub fn load_map(&mut self, path: &String) -> io::Result<()> {
        let f = BufReader::new(File::open(path)?);
        for (y, line) in f.lines().enumerate() {
            if y as i32 >= self.height {
                continue;
            }
            for (x, c) in line?.chars().enumerate() {
                if x as i32 >= self.width {
                    break;
                }
                match c {
                    '#' => {
                        self.blocks.insert(Block {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '@' => {
                        self.food_exists = true;
                        self.food_x = x as i32;
                        self.food_y = y as i32;
                    }
                    ' ' => {}
                    _ => unimplemented!("Unexpected symbol"),
                }
            }
        }
        Ok(())
    }
}
