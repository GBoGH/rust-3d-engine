use super::vector::Vector2Color;
use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Color;
use crossterm::style::{Print, SetForegroundColor};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub struct Display {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vector2Color>,
    pub edge_buffer: Vec<Vector2Color>,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = Vec::new();
        let edge_buffer = Vec::new();
        execute!(stdout(), Hide).unwrap();
        Display {
            width,
            height,
            buffer,
            edge_buffer,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.edge_buffer.clear();
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.buffer.push(Vector2Color(x as f32, y as f32, color));
    }

    fn draw_pixel_to_terminal(pixel: &Vector2Color, real_color: Color) {
        execute!(
            stdout(),
            MoveTo(pixel.0 as u16, pixel.1 as u16),
            SetForegroundColor(real_color),
            Print(Self::get_character_by_color(pixel.2)),
        )
        .unwrap();
    }

    fn draw_edges(&self) {
        for pixel in self.edge_buffer.iter() {
            Self::draw_pixel_to_terminal(&pixel, Color::DarkGrey);
        }
    }
    fn draw_vertices(&self) {
        for pixel in self.buffer.iter() {
            Self::draw_pixel_to_terminal(&pixel, Color::Grey);
        }
    }

    fn populate_edges(&mut self) {
        for pixel1 in self.buffer.iter() {
            for pixel2 in self.buffer.iter() {
                if pixel1.0 == pixel2.0 && pixel1.1 == pixel2.1 {
                    continue;
                }
                for x in (pixel1.0.round() as i32)..(pixel2.0.round() as i32) {
                    let x_coord = x as f32;
                    let y_coord = (pixel1.1 as f32)
                        + ((pixel2.1 as f32) - (pixel1.1 as f32))
                            * ((x_coord - (pixel1.0 as f32))
                                / ((pixel2.0 as f32) - (pixel1.0 as f32)));
                    let color = 1;

                    let mut is_vertex = false;

                    for pixel in self.buffer.iter() {
                        if pixel.0 == x_coord && pixel.1 == y_coord {
                            is_vertex = true;
                            break;
                        }
                    }
                    if !is_vertex {
                        self.edge_buffer
                            .push(Vector2Color(x_coord, y_coord as f32, color));
                    }
                }

                for y in (pixel1.1.round() as i32)..(pixel2.1.round() as i32) {
                    let y_coord = y as f32;
                    let x_coord = (pixel1.0 as f32)
                        + ((pixel2.0 as f32) - (pixel1.0 as f32))
                            * ((y_coord - (pixel1.1 as f32))
                                / ((pixel2.1 as f32) - (pixel1.1 as f32)));
                    let color = 1;

                    let mut is_vertex = false;

                    for pixel in self.buffer.iter() {
                        if pixel.0 == x_coord && pixel.1 == y_coord {
                            is_vertex = true;
                            break;
                        }
                    }
                    if !is_vertex {
                        self.edge_buffer
                            .push(Vector2Color(x_coord, y_coord as f32, color));
                    }
                }
            }
        }
    }

    pub fn clear_terminal(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn draw_to_terminal(&mut self) {
        self.clear_terminal();
        self.draw_vertices();
        self.populate_edges();
        self.draw_edges();
    }

    fn get_color_map() -> &'static str {
        " .'`^,:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
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

        if rounded_x > 0.0
            && rounded_x < self.width as f32
            && rounded_y > 0.0
            && rounded_y < self.height as f32
        {
            self.set_pixel(rounded_x as usize, rounded_y as usize, color);
        }
    }
}
