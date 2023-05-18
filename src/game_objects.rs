use crate::shapes::{Circle, Line, Point, Triangle};

use rand::Rng;

use std::f32::consts::PI;
use std::time::Instant;

// -----------------------------------------------------------------------------

const WRAPAROUND_OFFSET_OFFSET: f32 = 1.0;

// -----------------------------------------------------------------------------
// Asteroid
// -----------------------------------------------------------------------------

pub const MAX_ASTEROIDS: usize = 52;
pub const ASTEROID_RADIUS_SMALL: f32 = 2.0;
pub const ASTEROID_RADIUS_MEDIUM: f32 = 4.0;
pub const ASTEROID_RADIUS_LARGE: f32 = 6.0;
const ASTEROID_MIN_SPEED: f32 = 10.0;
pub const ASTEROID_MAX_SPEED_LOWER_LIMIT: f32 = 20.0;
pub const ASTEROID_MAX_SPEED_HIGHER_LIMIT: f32 = 40.0;
const ASTEROID_HEALTH_SMALL: i8 = 25;
const ASTEROID_HEALTH_MEDIUM: i8 = 50;
const ASTEROID_HEALTH_LARGE: i8 = 75;

pub struct Asteroids {
    pub exists: [bool; MAX_ASTEROIDS],
    pub hp: [i8; MAX_ASTEROIDS],
    pub circle: [Circle; MAX_ASTEROIDS],
    pub velocity: [Point; MAX_ASTEROIDS],
    pub max_speed: f32,
}

impl Default for Asteroids {
    fn default() -> Self {
        Asteroids {
            exists: [Default::default(); MAX_ASTEROIDS],
            hp: [Default::default(); MAX_ASTEROIDS],
            circle: [Default::default(); MAX_ASTEROIDS],
            velocity: [Default::default(); MAX_ASTEROIDS],
            max_speed: ASTEROID_MAX_SPEED_LOWER_LIMIT,
        }
    }
}

#[derive(Copy, Clone)]
pub enum AsteroidSize {
    SMALL,
    MEDIUM,
    LARGE,
}

impl AsteroidSize {
    pub fn from_radius(radius: f32) -> Result<AsteroidSize, String> {
        if radius == ASTEROID_RADIUS_SMALL {
            Ok(AsteroidSize::SMALL)
        } else if radius == ASTEROID_RADIUS_MEDIUM {
            Ok(AsteroidSize::MEDIUM)
        } else if radius == ASTEROID_RADIUS_LARGE {
            Ok(AsteroidSize::LARGE)
        } else {
            Err(format!("Invalid radius {radius}"))
        }
    }
}

impl Asteroids {
    pub fn create(
        &mut self,
        size: AsteroidSize,
        position: Point,
        velocity: Point,
    ) -> Result<usize, String> {
        // Create it in the fist position where exists is false
        if let Some(index) = self.exists.iter().position(|&e| !e) {
            self.exists[index] = true;
            self.circle[index] = Circle {
                center: position,
                radius: match size {
                    AsteroidSize::SMALL => ASTEROID_RADIUS_SMALL,
                    AsteroidSize::MEDIUM => ASTEROID_RADIUS_MEDIUM,
                    AsteroidSize::LARGE => ASTEROID_RADIUS_LARGE,
                },
            };
            self.velocity[index] = velocity;
            self.hp[index] = match size {
                AsteroidSize::SMALL => ASTEROID_HEALTH_SMALL,
                AsteroidSize::MEDIUM => ASTEROID_HEALTH_MEDIUM,
                AsteroidSize::LARGE => ASTEROID_HEALTH_LARGE,
            };
            Ok(index)
        } else {
            Err("No space left to create asteroid.".to_string())
        }
    }

    pub fn create_at_border(&mut self, max_coords: Point) -> Result<usize, String> {
        // Create it somewhere right at the border of the game canvas
        let border = rand::thread_rng().gen_range(0..=3);
        let mut x = rand::random::<f32>() * max_coords.x;
        let mut y = rand::random::<f32>() * max_coords.y;
        if border == 0 {
            x = -ASTEROID_RADIUS_LARGE;
        } else if border == 1 {
            y = -ASTEROID_RADIUS_LARGE;
        } else if border == 2 {
            x = max_coords.x + ASTEROID_RADIUS_LARGE;
        } else if border == 3 {
            y = max_coords.y + ASTEROID_RADIUS_LARGE;
        }
        let speed =
            ASTEROID_MIN_SPEED + rand::random::<f32>() * (self.max_speed - ASTEROID_MIN_SPEED);
        let angle = rand::random::<f32>() * 2.0 * PI;
        self.create(
            AsteroidSize::LARGE,
            Point { x, y },
            Point::from_polar(speed, angle),
        )
    }

    pub fn update_positions(&mut self, max_coords: Point, dt: f32) {
        for i in 0..MAX_ASTEROIDS {
            if !self.exists[i] {
                continue;
            }
            self.circle[i].update_position_wraparound(
                self.velocity[i],
                max_coords,
                self.circle[i].radius * WRAPAROUND_OFFSET_OFFSET,
                dt,
            );
        }
    }

    pub fn none_exist(&self) -> bool {
        !self.exists.iter().any(|&e| e)
    }

    pub fn clear(&mut self) {
        self.exists.iter_mut().for_each(|e| *e = false)
    }
}

// -----------------------------------------------------------------------------
// Bullet
// -----------------------------------------------------------------------------

pub const MAX_BULLETS: usize = 32;
const BULLET_SPEED: f32 = 200.0;
const BULLET_LENGTH: f32 = 0.75;
pub const BULLET_DAMAGE: i8 = 25;

pub struct Bullets {
    pub exists: [bool; MAX_BULLETS],
    pub line_segment: [Line; MAX_BULLETS],
    pub velocity: [Point; MAX_BULLETS],
}

impl Default for Bullets {
    fn default() -> Self {
        Bullets {
            exists: [Default::default(); MAX_BULLETS],
            line_segment: [Default::default(); MAX_BULLETS],
            velocity: [Default::default(); MAX_BULLETS],
        }
    }
}

impl Bullets {
    pub fn clear(&mut self) {
        self.exists.iter_mut().for_each(|e| *e = false)
    }

    pub fn create(&mut self, position: Point, angle: f32) -> Result<usize, String> {
        // Create it in the fist position where exists is false
        if let Some(index) = self.exists.iter().position(|&e| !e) {
            self.exists[index] = true;
            self.line_segment[index] = Line {
                p1: position + Point::from_polar(BULLET_LENGTH, angle),
                p2: position,
            };
            self.velocity[index] = Point::from_polar(BULLET_SPEED, angle);
            Ok(index)
        } else {
            Err("No space left to create bullet.".to_string())
        }
    }

    pub fn update_positions(&mut self, max_coords: Point, dt: f32) {
        for i in 0..MAX_BULLETS {
            if !self.exists[i] {
                continue;
            }
            self.line_segment[i].update_position(self.velocity[i], dt);
            self.exists[i] = !(self.line_segment[i].p2.x < 0.0
                || self.line_segment[i].p2.x > max_coords.x
                || self.line_segment[i].p2.y < 0.0
                || self.line_segment[i].p2.y > max_coords.y);
        }
    }
}

// -----------------------------------------------------------------------------
// Ship
// -----------------------------------------------------------------------------

pub const MAX_SHIPS: usize = 2;
const SHIP_WIDTH: f32 = 4.0;
const SHIP_LENGTH: f32 = 5.0;
pub const SHIP_MASS: f32 = 0.5;
const SHIP_SPEED_MAX: f32 = 100.0;
const SHIP_ACCELERATION_LEVEL1: f32 = 15.0;
const SHIP_ACCELERATION_LEVEL2: f32 = 30.0;
const SHIP_ACCELERATION_LEVEL3: f32 = 45.0;
const SHIP_DEACCELERATION: f32 = 2.5;
const SHIP_MAX_ANGULAR_VELOCITY: f32 = PI * 1.5;
const SHIP_ANGULAR_ACCELERATION_LEVEL1: f32 = PI * 8.0;
const SHIP_ANGULAR_ACCELERATION_LEVEL2: f32 = PI * 8.0;
const SHIP_ANGULAR_ACCELERATION_LEVEL3: f32 = PI * 8.0;
const SHIP_GUN_FIRE_RATE_NS_LEVEL1: u128 = 400000000;
const SHIP_GUN_FIRE_RATE_NS_LEVEL2: u128 = 300000000;
const SHIP_GUN_FIRE_RATE_NS_LEVEL3: u128 = 200000000;

#[derive(Copy, Clone, Default)]
pub enum UpgradeLevel {
    #[default]
    LEVEL1,
    LEVEL2,
    LEVEL3,
}

pub struct Ships {
    pub exists: [bool; MAX_SHIPS],
    pub hp: [i8; MAX_SHIPS],
    pub triangle: [Triangle; MAX_SHIPS],
    pub velocity: [Point; MAX_SHIPS],
    acceleration: [f32; MAX_SHIPS],
    pub back_thruster_level: [UpgradeLevel; MAX_SHIPS],
    pub angular_velocity: [f32; MAX_SHIPS],
    angular_acceleration: [f32; MAX_SHIPS],
    pub side_thrusters_level: [UpgradeLevel; MAX_SHIPS],
    side_thruster_pressed: [[bool; 2]; MAX_SHIPS],
    pub gun_level: [UpgradeLevel; MAX_SHIPS],
    pub gun_auto: [bool; MAX_SHIPS],
    gun_trigger_pressed: [bool; MAX_SHIPS],
    gun_trigger_released: [bool; MAX_SHIPS],
    gun_last_fired_t: [Instant; MAX_SHIPS],
    // laser? dmg 25 per [unit of time], ends at edge. rate of heating (fast), rate of cooling (slow), rate of cooling after overheating (slower)
    // bomb? -> radius? damage -> 100, ends at edge. replenish rate (slow). speed (slower than bullet)?
    // shield -> hp? replenish rate? deplenish rate?
}

impl Default for Ships {
    fn default() -> Self {
        Ships {
            exists: [Default::default(); MAX_SHIPS],
            hp: [Default::default(); MAX_SHIPS],
            triangle: [Default::default(); MAX_SHIPS],
            velocity: [Default::default(); MAX_SHIPS],
            acceleration: [Default::default(); MAX_SHIPS],
            back_thruster_level: [Default::default(); MAX_SHIPS],
            angular_velocity: [Default::default(); MAX_SHIPS],
            angular_acceleration: [Default::default(); MAX_SHIPS],
            side_thrusters_level: [Default::default(); MAX_SHIPS],
            side_thruster_pressed: [Default::default(); MAX_SHIPS],
            gun_level: [Default::default(); MAX_SHIPS],
            gun_last_fired_t: [Instant::now(); MAX_SHIPS],
            gun_trigger_pressed: [Default::default(); MAX_SHIPS],
            gun_trigger_released: [Default::default(); MAX_SHIPS],
            gun_auto: [Default::default(); MAX_SHIPS],
        }
    }
}

#[derive(Copy, Clone)]
pub enum RotationDirection {
    COUNTERCLOCKWISE,
    CLOCKWISE,
}

impl RotationDirection {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Ships {
    pub fn none_exist(&self) -> bool {
        !self.exists.iter().any(|&e| e)
    }

    pub fn create(&mut self, position: Point) -> Result<usize, String> {
        // Create it in the fist position where exists is false
        if let Some(index) = self.exists.iter().position(|&e| !e) {
            self.exists[index] = true;
            self.hp[index] = 100;
            self.triangle[index] = Triangle {
                v1: Point {
                    x: SHIP_WIDTH / 2.0,
                    y: -SHIP_LENGTH,
                },
                v2: Point {
                    x: SHIP_WIDTH,
                    y: 0.0,
                },
                v3: Point { x: 0.0, y: 0.0 },
            };
            let delta = position - self.triangle[index].circumcenter();
            self.triangle[index].update_position(delta, 1.0);
            self.velocity[index] = Point { x: 0.0, y: 0.0 };
            self.acceleration[index] = 0.0;
            self.back_thruster_level[index] = UpgradeLevel::LEVEL1;
            self.angular_velocity[index] = 0.0;
            self.angular_acceleration[index] = 0.0;
            self.side_thrusters_level[index] = UpgradeLevel::LEVEL1;
            self.side_thruster_pressed[index] = [false, false];
            self.gun_level[index] = UpgradeLevel::LEVEL1;
            self.gun_last_fired_t[index] = Instant::now();
            self.gun_trigger_pressed[index] = false;
            self.gun_trigger_released[index] = true;
            self.gun_auto[index] = false;
            Ok(index)
        } else {
            Err("No space left to create ship.".to_string())
        }
    }

    pub fn gun_trigger_pressed(&mut self, player: Player) {
        let index = player.to_index();
        if self.gun_trigger_released[index] {
            self.gun_trigger_pressed[index] = true;
            self.gun_trigger_released[index] = false;
        }
    }

    pub fn gun_trigger_released(&mut self, player: Player) {
        let index = player.to_index();
        self.gun_trigger_pressed[index] = false;
        self.gun_trigger_released[index] = true;
    }

    pub fn accelerator_pressed(&mut self, player: Player) {
        let index = player.to_index();
        self.acceleration[index] = match self.back_thruster_level[index] {
            UpgradeLevel::LEVEL1 => SHIP_ACCELERATION_LEVEL1,
            UpgradeLevel::LEVEL2 => SHIP_ACCELERATION_LEVEL2,
            UpgradeLevel::LEVEL3 => SHIP_ACCELERATION_LEVEL3,
        }
    }

    pub fn accelerator_released(&mut self, player: Player) {
        let index = player.to_index();
        self.acceleration[index] = 0.0;
    }

    pub fn side_thruster_pressed(&mut self, player: Player, direction: RotationDirection) {
        let index = player.to_index();
        let dir_index = direction.to_index();
        if !self.side_thruster_pressed[index][dir_index] {
            self.angular_acceleration[index] += match self.side_thrusters_level[index] {
                UpgradeLevel::LEVEL1 => SHIP_ANGULAR_ACCELERATION_LEVEL1,
                UpgradeLevel::LEVEL2 => SHIP_ANGULAR_ACCELERATION_LEVEL2,
                UpgradeLevel::LEVEL3 => SHIP_ANGULAR_ACCELERATION_LEVEL3,
            } * match direction {
                RotationDirection::COUNTERCLOCKWISE => -1.0,
                RotationDirection::CLOCKWISE => 1.0,
            };
            self.side_thruster_pressed[index][dir_index] = true;
        }
    }

    pub fn side_thruster_released(&mut self, player: Player, direction: RotationDirection) {
        let index = player.to_index();
        let dir_index = direction.to_index();
        self.angular_acceleration[index] -= match self.side_thrusters_level[index] {
            UpgradeLevel::LEVEL1 => SHIP_ANGULAR_ACCELERATION_LEVEL1,
            UpgradeLevel::LEVEL2 => SHIP_ANGULAR_ACCELERATION_LEVEL2,
            UpgradeLevel::LEVEL3 => SHIP_ANGULAR_ACCELERATION_LEVEL3,
        } * match direction {
            RotationDirection::COUNTERCLOCKWISE => -1.0,
            RotationDirection::CLOCKWISE => 1.0,
        };
        self.side_thruster_pressed[index][dir_index] = false;
        if self.side_thruster_pressed[index].iter().all(|&x| !x) {
            self.angular_acceleration[index] = 0.0;
        }
    }

    pub fn update_positions(&mut self, max_coords: Point, dt: f32) {
        for i in 0..MAX_SHIPS {
            if !self.exists[i] {
                continue;
            }
            // Update angular velocity
            if self.angular_acceleration[i] == 0.0 && self.angular_velocity[i] != 0.0 {
                let prev_signum = self.angular_velocity[i].signum();
                self.angular_velocity[i] -= self.angular_velocity[i].signum()
                    * match self.side_thrusters_level[i] {
                        UpgradeLevel::LEVEL1 => SHIP_ANGULAR_ACCELERATION_LEVEL1,
                        UpgradeLevel::LEVEL2 => SHIP_ANGULAR_ACCELERATION_LEVEL2,
                        UpgradeLevel::LEVEL3 => SHIP_ANGULAR_ACCELERATION_LEVEL3,
                    }
                    * dt;
                if prev_signum != self.angular_velocity[i].signum() {
                    self.angular_velocity[i] = 0.0;
                }
            } else {
                self.angular_velocity[i] += self.angular_acceleration[i] * dt;
            }
            if self.angular_velocity[i].abs() > SHIP_MAX_ANGULAR_VELOCITY {
                self.angular_velocity[i] =
                    SHIP_MAX_ANGULAR_VELOCITY * self.angular_velocity[i].signum();
            }
            // Update angle
            self.triangle[i].rotate_around_circumcenter(self.angular_velocity[i], dt);
            // Update velocity
            if self.acceleration[i] == 0.0 && self.velocity[i].magnitude_squared() != 0.0 {
                self.velocity[i] -= self.velocity[i].normalized() * SHIP_DEACCELERATION * dt;
            } else {
                self.velocity[i] += self.triangle[i].direction() * self.acceleration[i] * dt;
            }
            let speed = self.velocity[i].magnitude();
            if speed > SHIP_SPEED_MAX {
                self.velocity[i] *= SHIP_SPEED_MAX / speed;
            }
            // Update position
            let wraparound_offset = self.triangle[i].circumradius() * WRAPAROUND_OFFSET_OFFSET;
            self.triangle[i].update_position_wraparound(
                self.velocity[i],
                max_coords,
                wraparound_offset,
                dt,
            )
        }
    }

    pub fn update_shooting(&mut self, bullets: &mut Bullets) {
        for i in 0..MAX_SHIPS {
            if self.exists[i]
                && self.gun_trigger_pressed[i]
                && self.gun_last_fired_t[i].elapsed().as_nanos()
                    >= match self.gun_level[i] {
                        UpgradeLevel::LEVEL1 => SHIP_GUN_FIRE_RATE_NS_LEVEL1,
                        UpgradeLevel::LEVEL2 => SHIP_GUN_FIRE_RATE_NS_LEVEL2,
                        UpgradeLevel::LEVEL3 => SHIP_GUN_FIRE_RATE_NS_LEVEL3,
                    }
            {
                bullets
                    .create(self.triangle[i].v1, self.triangle[i].angle())
                    .unwrap();
                if !self.gun_auto[i] {
                    self.gun_trigger_pressed[i] = false;
                }
                self.gun_last_fired_t[i] = Instant::now();
            }
        }
    }
}

// -----------------------------------------------------------------------------

// Enemy ships. Behavior? Patterns? -> random? seek? combines? pacman? weapons? shields? evade? how intelligent? up to how many at the same time?

// -----------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum Player {
    PLAYER1,
    PLAYER2,
    PLAYER3,
    PLAYER4,
}

impl Player {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}
