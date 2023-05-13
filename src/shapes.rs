use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// -----------------------------------------------------------------------------

// Point
#[derive(Default, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        Point {
            x: self * point.x,
            y: self * point.y,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = *self - other;
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar;
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Point {
    pub fn update_position(&mut self, delta: Point, dt: f32) {
        *self += delta * dt;
    }

    pub fn update_position_wraparound(
        &mut self,
        delta: Point,
        max_coords: Point,
        wraparound_offset: f32,
        dt: f32,
    ) {
        self.update_position(delta, dt);
        let max_x = max_coords.x + wraparound_offset;
        let max_y = max_coords.y + wraparound_offset;
        if self.x > max_x {
            self.x = -wraparound_offset;
        } else if self.x < -wraparound_offset {
            self.x = max_x;
        }
        if self.y > max_y {
            self.y = -wraparound_offset;
        } else if self.y < -wraparound_offset {
            self.y = max_y;
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn from_polar(r: f32, t: f32) -> Point {
        let (sin, cos) = t.sin_cos();
        Point {
            x: r * cos,
            y: r * sin,
        }
    }

    pub fn normalized(&self) -> Point {
        *self * self.magnitude().recip()
    }

    pub fn perpendicular(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn rotated(&self, angle: f32) -> Point {
        let (sin, cos) = angle.sin_cos();
        Point {
            x: cos * self.x - sin * self.y,
            y: sin * self.x + cos * self.y,
        }
    }
}

// -----------------------------------------------------------------------------

// Line
#[derive(Default, Copy, Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn update_position(&mut self, delta: Point, dt: f32) {
        self.p1.update_position(delta, dt);
        self.p2.update_position(delta, dt);
    }
}

// -----------------------------------------------------------------------------

// Circle
#[derive(Default, Copy, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub fn update_position(&mut self, delta: Point, dt: f32) {
        self.center.update_position(delta, dt);
    }

    pub fn update_position_wraparound(
        &mut self,
        delta: Point,
        max_coords: Point,
        wraparound_offset: f32,
        dt: f32,
    ) {
        self.center
            .update_position_wraparound(delta, max_coords, wraparound_offset, dt);
    }
}

// -----------------------------------------------------------------------------

// Triangle
#[derive(Default, Copy, Clone)]
pub struct Triangle {
    pub v1: Point,
    pub v2: Point,
    pub v3: Point,
}

impl Triangle {
    pub fn update_position(&mut self, delta: Point, dt: f32) {
        let total_delta = delta * dt;
        self.v1 += total_delta;
        self.v2 += total_delta;
        self.v3 += total_delta;
    }

    pub fn update_position_wraparound(
        &mut self,
        delta: Point,
        max_coords: Point,
        wraparound_offset: f32,
        dt: f32,
    ) {
        let c = self.circumcenter();
        let mut c_updated = c;
        c_updated.update_position_wraparound(delta, max_coords, wraparound_offset, dt);
        let displacement = c_updated - c;
        self.update_position(displacement, 1.0);
    }

    pub fn rotate_around_circumcenter(&mut self, delta: f32, dt: f32) {
        let c = self.circumcenter();
        let rotate_vertex_around_circumcenter = |v: Point| (v - c).rotated(delta * dt) + c;
        self.v1 = rotate_vertex_around_circumcenter(self.v1);
        self.v2 = rotate_vertex_around_circumcenter(self.v2);
        self.v3 = rotate_vertex_around_circumcenter(self.v3);
    }

    pub fn angle(&self) -> f32 {
        (self.v3 - self.v2).perpendicular().angle()
    }

    pub fn centroid(&self) -> Point {
        (self.v1 + self.v2 + self.v3) / 3.0
    }

    pub fn direction(&self) -> Point {
        (self.v3 - self.v2).perpendicular().normalized()
    }

    pub fn shortest_vertex_to_circumcenter_distance(&self) -> f32 {
        let c = self.circumcenter();
        let d1 = (c - self.v1).magnitude_squared();
        let d2 = (c - self.v2).magnitude_squared();
        let d3 = (c - self.v3).magnitude_squared();
        d1.min(d2).min(d3).sqrt()
    }

    pub fn circumradius(&self) -> f32 {
        let a = (self.v1 - self.v2).magnitude();
        let b = (self.v2 - self.v3).magnitude();
        let c = (self.v3 - self.v1).magnitude();
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        let radius = (a * b * c) / (4.0 * area);
        radius
    }

    pub fn circumcenter(&self) -> Point {
        let a = self.v1.x - self.v2.x;
        let b = self.v1.y - self.v2.y;
        let c = self.v1.x - self.v3.x;
        let d = self.v1.y - self.v3.y;
        let e = (self.v1.x * self.v1.x - self.v2.x * self.v2.x)
            + (self.v1.y * self.v1.y - self.v2.y * self.v2.y);
        let f = (self.v1.x * self.v1.x - self.v3.x * self.v3.x)
            + (self.v1.y * self.v1.y - self.v3.y * self.v3.y);
        let x = (b * f - d * e) / (2.0 * (b * c - a * d));
        let y = (a * f - c * e) / (2.0 * (a * d - b * c));
        Point { x, y }
    }

    pub fn circumcircle(&self) -> Circle {
        Circle {
            center: self.circumcenter(),
            radius: self.circumradius(),
        }
    }
}
