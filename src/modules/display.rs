use rand;
use rand::Rng;
use std::io::stdout;

use crossterm::{terminal::{Clear, ClearType}, execute};
use super::vector::Vector2Color;

pub struct Display {
    pub width: usize,
    pub height: usize,

    pub buffer: Vec<Vec<u8>>,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![0; width as usize]; height as usize];
        Display {
            width,
            height,
            buffer,
        }
    }

    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.buffer[y][x] = color;
    }

    pub fn clear_terminal(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn draw_to_terminal(&self) {
        self.clear_terminal();
        for row in self.buffer.iter() {
            for pixel in row.iter() {
                print!("{}", Self::get_character_by_color(*pixel));
            }
            println!();
        }
    }

    fn get_color_map() -> &'static str {
        " .'`^,:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
    }

    pub fn random_buffer(&mut self) {
        let mut rng = rand::thread_rng();
        let character_count = Self::get_color_map().chars().count() - 1;

        for row in self.buffer.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = rng.gen_range(0..character_count) as u8;
            }
        }
    }

    fn get_character_by_color(color: u8) -> char {
        let character = Self::get_color_map().chars().nth(color as usize).unwrap();
        character
    }

    pub fn draw_point(&mut self, point: Vector2Color) {
        let Vector2Color(x, y, color) = point;
        let half_width = self.width as f32 / 2.0;
        let half_height = self.height as f32 / 2.0;
        let rounded_x = (x + half_width).round();
        let rounded_y = (y + half_height).round();

        if rounded_x > 0.0 && rounded_x < self.width as f32 && rounded_y > 0.0 && rounded_y < self.height as f32 {
            self.set_pixel(rounded_x as usize, rounded_y as usize, color);
        }
    }
}
