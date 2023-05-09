use crate::game_objects::{Asteroids, Bullets, Ships, MAX_ASTEROIDS, MAX_BULLETS, MAX_SHIPS};
use crate::shapes::Point;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer {
    pub canvas: Canvas<Window>,
    pub max_coords: Point,
    scaling_factor: f32,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>) -> Renderer {
        Renderer {
            canvas,
            max_coords: Point::default(),
            scaling_factor: 0.0,
        }
    }

    pub fn update_max_coords(&mut self) {
        let canvas_size = self.canvas.output_size().unwrap();
        let x = canvas_size.0 as f32;
        let y = canvas_size.1 as f32;
        let (max_x, max_y) = if x >= y { (x / y, 1.0) } else { (1.0, y / x) };
        self.max_coords = Point {
            x: max_x * 100.0,
            y: max_y * 100.0,
        };
    }

    pub fn update_scaling_factor(&mut self) {
        let canvas_size = self.canvas.output_size().unwrap();
        let x = canvas_size.0 as f32;
        let y = canvas_size.1 as f32;
        self.scaling_factor = if x >= y { y } else { x } / 100.0
    }

    pub fn render_asteroids(&self, asteroids: &Asteroids) {
        for i in 0..MAX_ASTEROIDS {
            if asteroids.exists[i] {
                self.canvas
                    .aa_circle(
                        (asteroids.circle[i].center.x * self.scaling_factor) as i16,
                        (asteroids.circle[i].center.y * self.scaling_factor) as i16,
                        (asteroids.circle[i].radius * self.scaling_factor) as i16,
                        Color::WHITE,
                    )
                    .unwrap();
            }
        }
    }

    pub fn render_bullets(&self, bullets: &Bullets) {
        for i in 0..MAX_BULLETS {
            if bullets.exists[i] {
                self.canvas
                    .thick_line(
                        (bullets.line_segment[i].p1.x * self.scaling_factor) as i16,
                        (bullets.line_segment[i].p1.y * self.scaling_factor) as i16,
                        (bullets.line_segment[i].p2.x * self.scaling_factor) as i16,
                        (bullets.line_segment[i].p2.y * self.scaling_factor) as i16,
                        2,
                        Color::WHITE,
                    )
                    .unwrap();
            }
        }
    }

    pub fn render_ships(&self, ships: &Ships) {
        for i in 0..MAX_SHIPS {
            if ships.exists[i] {
                self.canvas
                    .aa_trigon(
                        (ships.triangle[i].v1.x * self.scaling_factor) as i16,
                        (ships.triangle[i].v1.y * self.scaling_factor) as i16,
                        (ships.triangle[i].v2.x * self.scaling_factor) as i16,
                        (ships.triangle[i].v2.y * self.scaling_factor) as i16,
                        (ships.triangle[i].v3.x * self.scaling_factor) as i16,
                        (ships.triangle[i].v3.y * self.scaling_factor) as i16,
                        Color::WHITE,
                    )
                    .unwrap();
                let c = ships.triangle[i].circumcircle();
                self.canvas
                    .aa_circle(
                        (c.center.x * self.scaling_factor) as i16,
                        (c.center.y * self.scaling_factor) as i16,
                        (c.radius * self.scaling_factor) as i16,
                        Color::WHITE,
                    )
                    .unwrap();
            }
        }
    }
}
