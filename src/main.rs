mod game_objects;
mod intersect;
mod physics;
mod render;
mod shapes;

use game_objects::{
    Asteroids, Bullets, Player, RotationDirection, Ships, ASTEROID_MAX_SPEED_HIGHER_LIMIT,
    ASTEROID_MAX_SPEED_LOWER_LIMIT,
};
use physics::{asteroid_asteroid_collisions, asteroid_bullet_collisions, asteroid_ship_collisions};
use render::Renderer;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::pixels::Color;

use std::thread::sleep;
use std::time::{Duration, Instant};

const TARGET_FRAME_RATE: f64 = 120.0; // Frames per second, must be more than 1
const PERIOD: f64 = 1000000000.0 / TARGET_FRAME_RATE; // Nanoseconds

pub fn main() {
    // Setup sdl2 objects
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rusteroids", 800, 600)
        .position_centered()
        .resizable()
        //.maximized()
        //.fullscreen_desktop()
        //.fullscreen()
        .build()
        .unwrap();

    let canvas = window
        .into_canvas()
        .accelerated()
        //.present_vsync()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer = Renderer::new(canvas);
    renderer.canvas.set_draw_color(Color::BLACK);
    renderer.canvas.clear();
    renderer.canvas.present();
    renderer.update_max_coords();
    renderer.update_scaling_factor();

    // Constants
    const STARTING_ASTEROIDS_COUNT: i32 = 3;

    // Game objects and variables
    let mut asteroids = Box::new(Asteroids::default());
    let mut bullets = Box::new(Bullets::default());
    let mut ships = Box::new(Ships::default());
    ships.create(renderer.max_coords / 2.0).unwrap();
    let mut level = 0;
    let mut start = Instant::now();
    let mut sleep_time_offset = 0.0;

    // Game loop
    'running: loop {
        let elapsed = start.elapsed().as_nanos();
        start = Instant::now();
        let dt = elapsed as f64;
        let dt_secs = (dt / 1000000000.0) as f32;

        // Events
        // let keyboard = KeyboardState::new(&event_pump);
        // if keyboard.is_scancode_pressed(Scancode::Space) {
        //     if shoot {
        //         ships.shoot(0, &mut bullets).unwrap();
        //         shoot = false;
        //     }
        // }
        // if keyboard.is_scancode_pressed(Scancode::Left) {
        //     ships
        //         .rotate(0, ShipRotateDirection::COUNTERCLOCKWISE, dt_secs)
        //         .unwrap();
        // }
        // if keyboard.is_scancode_pressed(Scancode::Right) {
        //     ships
        //         .rotate(0, ShipRotateDirection::CLOCKWISE, dt_secs)
        //         .unwrap();
        // }
        // if keyboard.is_scancode_pressed(Scancode::Up) {
        //     ships.accelerate(0, dt_secs).unwrap();
        // }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::SizeChanged(_, _) => {
                        renderer.update_max_coords();
                        renderer.update_scaling_factor();
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => ships.gun_trigger_pressed(Player::PLAYER1),

                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => ships.gun_trigger_released(Player::PLAYER1),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => ships.accelerator_pressed(Player::PLAYER1),

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => ships.accelerator_released(Player::PLAYER1),
                // Event::KeyDown {
                //     keycode: Some(Keycode::Left),
                //     ..
                // } => {
                //     ships.triangles[0].update_angle(PI, dt_secs);
                // }
                // Event::KeyDown {
                //     keycode: Some(Keycode::Right),
                //     ..
                // } => {
                //     ships.triangles[0].update_angle(-PI, dt_secs);
                // }
                _ => {}
            }
        }

        // Update game objects
        if asteroids.none_exist() {
            for _ in 0..(STARTING_ASTEROIDS_COUNT + level.min(10)) {
                asteroids.create_at_border(renderer.max_coords).unwrap();
                asteroids.max_speed = ASTEROID_MAX_SPEED_LOWER_LIMIT
                    + level.min(10) as f32
                        * (ASTEROID_MAX_SPEED_HIGHER_LIMIT - ASTEROID_MAX_SPEED_LOWER_LIMIT)
                        / 10.0;
            }
            level += 1;
        }
        ships.update_positions(renderer.max_coords, dt_secs);
        bullets.update_positions(renderer.max_coords, dt_secs);
        asteroids.update_positions(renderer.max_coords, dt_secs);
        ships.update_shooting(&mut bullets);

        // Physics
        asteroid_asteroid_collisions(&mut asteroids); // TODO: For testing, remove eventually
        asteroid_bullet_collisions(&mut asteroids, &mut bullets);
        asteroid_ship_collisions(&mut asteroids, &mut ships);

        // Render
        renderer.canvas.set_draw_color(Color::BLACK);
        renderer.canvas.clear();
        renderer.render_ships(&ships);
        renderer.render_bullets(&bullets);
        renderer.render_asteroids(&asteroids);
        renderer.canvas.present();

        // Try to maintain stable FPS
        let mut sleep_time = PERIOD;
        sleep_time_offset += 0.1 * (dt - PERIOD);
        sleep_time_offset = sleep_time_offset.max(1.0).min(PERIOD);
        sleep_time -= sleep_time_offset;
        sleep(Duration::new(0, sleep_time as u32));
    }
}
