use crate::shapes::{Circle, Line, Point, Triangle};

// -----------------------------------------------------------------------------

pub fn line_segment_circle_intersect(line_segment: &Line, circle: &Circle) -> (bool, Point) {
    let closest_point = closest_point_on_line_segment_to_other_point(line_segment, circle.center);
    (point_in_circle(closest_point, circle), closest_point)
}

pub fn triangle_circle_intersect(triangle: &Triangle, circle: &Circle) -> (bool, Point) {
    let (intersects1, closest1) = line_segment_circle_intersect(
        &Line {
            p1: triangle.v1,
            p2: triangle.v2,
        },
        circle,
    );
    let (intersects2, closest2) = line_segment_circle_intersect(
        &Line {
            p1: triangle.v2,
            p2: triangle.v3,
        },
        circle,
    );
    let (intersects3, closest3) = line_segment_circle_intersect(
        &Line {
            p1: triangle.v3,
            p2: triangle.v1,
        },
        circle,
    );
    let intersects = intersects1 || intersects2 || intersects3 ||
        // Rare case where the whole circule is inside the triangle
        point_in_triangle(triangle, circle.center);

    let d1 = (closest1 - circle.center).magnitude_squared();
    let d2 = (closest2 - circle.center).magnitude_squared();
    let d3 = (closest3 - circle.center).magnitude_squared();
    let mut closest = closest1;
    let mut d_min = d1;
    if d2 < d_min {
        closest = closest2;
        d_min = d2;
    }
    if d3 < d_min {
        closest = closest3;
    }
    return (intersects, closest);
}

pub fn circles_intersect(circle1: Circle, circle2: Circle) -> bool {
    let d = circle1.center - circle2.center;
    let sum_of_radii = circle1.radius + circle2.radius;
    d.magnitude_squared() <= sum_of_radii * sum_of_radii
}

fn point_in_circle(p: Point, circle: &Circle) -> bool {
    let d = p - circle.center;
    d.magnitude_squared() <= circle.radius * circle.radius
}

fn closest_point_on_line_segment_to_other_point(line_segment: &Line, p: Point) -> Point {
    let ab = line_segment.p2 - line_segment.p1;
    let ap = p - line_segment.p1;
    let projection = ap.x * ab.x + ap.y * ab.y;
    let norm_proj = projection / ab.magnitude_squared();
    let norm_proj_clamped = norm_proj.max(0.0).min(1.0);
    let closest_point = line_segment.p1 + (ab * norm_proj_clamped);
    closest_point
}

fn point_in_triangle(triangle: &Triangle, point: Point) -> bool {
    let s1 = triangle.v3.y - triangle.v1.y;
    let s2 = triangle.v3.x - triangle.v1.x;
    let s3 = triangle.v2.y - triangle.v1.y;
    let s4 = point.y - triangle.v1.y;
    let w1 = (triangle.v1.x * s1 + s4 * s2 - point.x * s1)
        / (s3 * s2 - (triangle.v2.x - triangle.v1.x) * s1);
    let w2 = (s4 - w1 * s3) / s1;
    w1 >= 0.0 && w2 >= 0.0 && (w1 + w2) <= 1.0
}
