// assume collision resolution takes place before velocity resolution

pub fn rects_collision_resolution_pos(
    pos: (f64, f64),
    vel: (f64, f64),
    rect: (f64, f64),
    other_pos: (f64, f64),
    other_rect: (f64, f64),
) -> (f64, f64) {
    'top_left: {
        let Some((intersection, dist)) =
            closest_point_rect_intersection(pos, vel, other_pos, other_rect)
        else {
            let (x, y) = pos;
            break 'top_left (x + vel.0, y + vel.1);
        };

        intersection
    }
}

fn point_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).abs().powi(2) + (a.1 - b.1).abs().powi(2)).sqrt()
}

fn horizontal_line_intersect(p0: (f64, f64), vel: (f64, f64), line_y: f64) -> Option<(f64, f64)> {
    if vel.1 == 0.0 {
        // exclusively going left or right, ergo there's no collision with a horizontal line
        return None;
    } else if vel.0 == 0.0 {
        // no change in x, ergo the intersect must be at p0_x
        return Some((p0.0, line_y));
    }
    // y = ax + b
    let a = vel.1 / vel.0;
    let b = p0.1 - p0.0 * a;

    let x = (line_y - b) / a;
    Some((x, line_y))
}

fn vertical_line_intersect(p0: (f64, f64), vel: (f64, f64), line_x: f64) -> Option<(f64, f64)> {
    if vel.0 == 0.0 {
        // exclusively going up or down, ergo there's no collision with a vertical line
        return None;
    } else if vel.1 == 0.0 {
        // no change in y, ergo the intersect must be at p0_y
        return Some((line_x, p0.1));
    }
    //
    // y = ax + b
    let a = vel.1 / vel.0;
    let b = p0.1 - p0.0 * a;

    let y = a * line_x + b;
    Some((line_x, y))
}

fn closest_point_rect_intersection(
    p0: (f64, f64),
    vel: (f64, f64),
    pos: (f64, f64),
    rect: (f64, f64),
) -> Option<((f64, f64), f64)> {
    [
        horizontal_line_intersect(p0, vel, pos.1),
        horizontal_line_intersect(p0, vel, pos.1 + rect.1),
        vertical_line_intersect(p0, vel, pos.0 + rect.0),
        vertical_line_intersect(p0, vel, pos.0),
    ]
    .into_iter()
    .flatten()
    .map(|point| (point, point_distance(p0, point)))
    .filter(|(_, dist)| *dist <= point_distance((0.0, 0.0), vel))
    .min_by(|(_, dist_a), (_, dist_b)| dist_a.total_cmp(dist_b))
}
