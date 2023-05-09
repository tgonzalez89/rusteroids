use crate::game_objects::{
    AsteroidSize, Asteroids, Bullets, Ships, ASTEROID_RADIUS_SMALL, BULLET_DAMAGE, MAX_ASTEROIDS,
    MAX_BULLETS, MAX_SHIPS, SHIP_MASS,
};
use crate::intersect::{
    circles_intersect, line_segment_circle_intersect, triangle_circle_intersect,
};
use crate::shapes::{Circle, Point};

use std::f32::consts::FRAC_PI_8;

// -----------------------------------------------------------------------------

const ASTEROID_COEFFICIENT_OF_RESTITUTION: f32 = 0.75;
const SHIP_COEFFICIENT_OF_RESTITUTION: f32 = 0.5;
const ASTEROID_SPLIT_SPEED_MULTIPLIER: f32 = 1.25;

// -----------------------------------------------------------------------------

fn calculate_collision_velocities(
    v1: Point,
    v2: Point,
    m1: f32,
    m2: f32,
    e: f32,
) -> (Point, Point) {
    let v1f = (e * m2 * (v2 - v1) + m1 * v1 + m2 * v2) / (m1 + m2);
    let v2f = (e * m1 * (v1 - v2) + m1 * v1 + m2 * v2) / (m1 + m2);
    (v1f, v2f)
}

fn displace_circles(circle1: &Circle, circle2: &Circle) -> (Point, Point) {
    let distance_between_centers = (circle2.center - circle1.center).magnitude();
    let sum_of_radii = circle1.radius + circle2.radius;
    let overlap_distance = sum_of_radii - distance_between_centers;
    let mut displacement = overlap_distance * (circle2.center - circle1.center) / 2.0;
    if distance_between_centers != 0.0 {
        displacement /= distance_between_centers;
    }
    (circle1.center - displacement, circle2.center + displacement)
}

pub fn asteroid_asteroid_collisions(asteroids: &mut Asteroids) {
    for i in 0..MAX_ASTEROIDS {
        if !asteroids.exists[i] {
            continue;
        }
        for j in (i + 1)..MAX_ASTEROIDS {
            if !asteroids.exists[j] {
                continue;
            }
            if circles_intersect(asteroids.circle[i], asteroids.circle[j]) {
                (asteroids.velocity[i], asteroids.velocity[j]) = calculate_collision_velocities(
                    asteroids.velocity[i],
                    asteroids.velocity[j],
                    1.0,
                    1.0,
                    ASTEROID_COEFFICIENT_OF_RESTITUTION,
                );
                (asteroids.circle[i].center, asteroids.circle[j].center) =
                    displace_circles(&asteroids.circle[i], &asteroids.circle[j]);
            }
        }
    }
}

pub fn asteroid_bullet_collisions(asteroids: &mut Asteroids, bullets: &mut Bullets) {
    let mut split = [false; MAX_ASTEROIDS];
    for i in 0..MAX_ASTEROIDS {
        if !asteroids.exists[i] {
            continue;
        }
        for j in 0..MAX_BULLETS {
            if !bullets.exists[j] {
                continue;
            }
            if line_segment_circle_intersect(&bullets.line_segment[j], &asteroids.circle[i]) {
                bullets.exists[j] = false;
                asteroids.hp[i] -= BULLET_DAMAGE;
                let destroy = asteroids.hp[i] <= 0;
                let is_small = asteroids.circle[i].radius == ASTEROID_RADIUS_SMALL;
                split[i] = destroy && !is_small;
                asteroids.exists[i] = !(destroy && is_small);
                if split[i] {
                    asteroids.velocity[i] *= ASTEROID_SPLIT_SPEED_MULTIPLIER;
                }
            }
        }
    }

    for i in 0..MAX_ASTEROIDS {
        if !split[i] {
            continue;
        }
        asteroids.exists[i] = false;
        let velocity1 = asteroids.velocity[i].clone().rotated(-FRAC_PI_8);
        let velocity2 = asteroids.velocity[i].clone().rotated(FRAC_PI_8);
        let new_radius = asteroids.circle[i].radius / 2.0;
        let (position1, position2) = displace_circles(
            &Circle {
                radius: new_radius,
                center: asteroids.circle[i].center + velocity1.normalized() * 0.001,
            },
            &Circle {
                radius: new_radius,
                center: asteroids.circle[i].center + velocity2.normalized() * 0.001,
            },
        );
        let new_radius_enum = AsteroidSize::from_radius(new_radius).unwrap();
        asteroids
            .create(new_radius_enum, position1, velocity1)
            .unwrap();
        asteroids
            .create(new_radius_enum, position2, velocity2)
            .unwrap();
    }
}

pub fn asteroid_ship_collisions(asteroids: &mut Asteroids, ships: &mut Ships) {
    for i in 0..MAX_ASTEROIDS {
        if !asteroids.exists[i] {
            continue;
        }
        for j in 0..MAX_SHIPS {
            if !ships.exists[j] {
                continue;
            }
            if triangle_circle_intersect(&ships.triangle[j], &asteroids.circle[i]) {
                (asteroids.velocity[i], ships.velocity[j]) = calculate_collision_velocities(
                    asteroids.velocity[i],
                    ships.velocity[j],
                    asteroids.circle[i].radius
                        * asteroids.circle[i].radius
                        * asteroids.circle[i].radius,
                    SHIP_MASS,
                    SHIP_COEFFICIENT_OF_RESTITUTION,
                );
                displace_circles(&ships.triangle[j].circumcircle(), &asteroids.circle[i]);
                // TODO: Better method of separating triangle and circle.
                //       Counter-rotate angular velocity? could help separating faster
                // while triangle_circle_intersect(&ships.triangle[j], &asteroids.circle[i]) {
                //     asteroids.circle[i].update_position(asteroids.velocity[i], 0.001);
                //     ships.triangle[j].update_position(ships.velocity[j], 0.001);
                // }
                asteroids.hp[i] -= 25;
                ships.hp[j] -= 25;

                // TODO:
                //       Damage based on velocity/kinetic energy?
                //ships.exist[j] = ships.hps[j] > 0; // TODO: enable ship destruction, in main.rs, restart game.
            }
        }
    }
}
