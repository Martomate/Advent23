use std::ops::Sub;

use num::Integer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Ray {
    start: Vec3,
    dir: Vec3,
}

impl Ray {
    fn new(start: Vec3, dir: Vec3) -> Self {
        Self { start, dir }
    }

    fn in_moving_frame(&self, velocity: Vec3) -> Self {
        Self {
            start: self.start,
            dir: self.dir - velocity,
        }
    }

    fn evaulate(&self, t: i64) -> Vec3 {
        Vec3::new(
            self.start.x + self.dir.x * t,
            self.start.y + self.dir.y * t,
            self.start.z + self.dir.z * t,
        )
    }
}

fn try_div_int(up: i64, down: i64) -> Option<i64> {
    let (d, r) = up.div_rem(&down);
    if r != 0 {
        return None;
    }
    Some(d)
}

/// Returns the intersection of the rays, but it has to happen in the future on integer coordinates.
fn future_collision_on_grid(r1: Ray, r2: Ray) -> Option<(i64, i64)> {
    let Ray { start: s1, dir: w1 } = r1;
    let Ray { start: s2, dir: w2 } = r2;

    // s1 + w1 t1 = s2 + w2 t2
    // w2 t2 + w1 t1 = ds

    let ds = s2 - s1;

    let down_xy = w2.x * w1.y - w1.x * w2.y;
    if down_xy != 0 {
        let t1_up = w2.x * ds.y - ds.x * w2.y;
        let t2_up = w1.x * ds.y - ds.x * w1.y;

        let t1 = try_div_int(t1_up, down_xy)?;
        let t2 = try_div_int(t2_up, down_xy)?;

        if Ray::new(s1, w1).evaulate(t1) != Ray::new(s2, w2).evaulate(t2) {
            return None;
        }
        return Some((t1, t2));
    }

    let down_yz = w2.y * w1.z - w1.y * w2.z;
    if down_yz != 0 {
        let t1_up = w2.y * ds.z - ds.y * w2.z;
        let t2_up = w1.y * ds.z - ds.y * w1.z;

        let t1 = try_div_int(t1_up, down_yz)?;
        let t2 = try_div_int(t2_up, down_yz)?;

        if Ray::new(s1, w1).evaulate(t1) != Ray::new(s2, w2).evaulate(t2) {
            return None;
        }
        return Some((t1, t2));
    }

    let down_zx = w2.z * w1.x - w1.z * w2.x;
    if down_zx != 0 {
        let t1_up = w2.z * ds.x - ds.z * w2.x;
        let t2_up = w1.z * ds.x - ds.z * w1.x;

        let t1 = try_div_int(t1_up, down_zx)?;
        let t2 = try_div_int(t2_up, down_zx)?;

        if Ray::new(s1, w1).evaulate(t1) != Ray::new(s2, w2).evaulate(t2) {
            return None;
        }
        return Some((t1, t2));
    }

    None
}

/// From the perspective of the flying rock all rays should converge at a point, but at different times.
/// The rock will see all rays approaching it from various directions at different timestamps.
/// This function returns those timestamps.
fn calculate_collision_times(rays: &[Ray], v: Vec3) -> Option<Vec<i64>> {
    let r0 = rays[0].in_moving_frame(v);
    let r1 = rays[1].in_moving_frame(v);
    let (t0, t1) = future_collision_on_grid(r0, r1)?;
    if t0 < 0 || t1 < 0 {
        return None;
    }

    let mut ts = Vec::new();
    ts.push(t0);
    ts.push(t1);

    let mut t_prev = t1;
    for i in 1..rays.len() - 1 {
        let r0 = rays[i].in_moving_frame(v);
        let r1 = rays[i + 1].in_moving_frame(v);
        let (t0, t1) = future_collision_on_grid(r0, r1)?;
        if t0 != t_prev {
            return None;
        }
        if t1 < 0 {
            return None;
        }
        t_prev = t1;
        ts.push(t1);
    }

    Some(ts)
}

fn verify_rays_converge(rays: &[Ray], a: Ray, ts: &[i64]) {
    for i in 0..rays.len() {
        let p1 = a.evaulate(ts[i]);
        let p2 = rays[i].evaulate(ts[i]);

        if p1 != p2 {
            // this should not happen!
            panic!("{p1:?} != {p2:?}");
        }
    }
}

fn parse_vec3(s: &str) -> Option<Vec3> {
    let mut ns = s.split(',').map(|s| s.trim().parse().unwrap());
    Some(Vec3::new(ns.next()?, ns.next()?, ns.next()?))
}

fn parse_input(lines: &[&str]) -> Vec<Ray> {
    lines
        .iter()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            Ray::new(
                parse_vec3(pos.trim()).unwrap(),
                parse_vec3(vel.trim()).unwrap(),
            )
        })
        .collect()
}

fn find_xy_intersection(r1: &Ray, r2: &Ray) -> Option<(f64, f64)> {
    let Ray { start: s1, dir: v1 } = *r1;
    let Ray { start: s2, dir: v2 } = *r2;

    // s1.x + t1 * v1.x = s2.x + t2 * v2.x
    // s1.y + t1 * v1.y = s2.y + t2 * v2.y

    // t1 = (ds.x * v2.y - ds.y * v2.x) / (v1.x * v2.y - v1.y * v2.x)
    // t2 = (ds.x * v1.y - ds.y * v1.x) / (v1.x * v2.y - v1.y * v2.x)

    let ds = s2 - s1;
    let t1_up = ds.x * v2.y - ds.y * v2.x;
    let t2_up = ds.x * v1.y - ds.y * v1.x;
    let down = v1.x * v2.y - v1.y * v2.x;

    if down == 0 {
        return None; // parallel lines
    }

    let t1 = t1_up as f64 / down as f64;
    let t2 = t2_up as f64 / down as f64;

    if t1 < 0.0 || t2 < 0.0 {
        return None; // crossed in the past
    }

    Some((
        s1.x as f64 + t1 * v1.x as f64,
        s1.y as f64 + t1 * v1.y as f64,
    ))
}

pub fn run(lines: Vec<&str>, lo: i64, hi: i64, part2: bool) -> u64 {
    let rays = parse_input(&lines);

    if !part2 {
        let mut count = 0;
        for i in 0..rays.len() {
            for j in (i + 1)..rays.len() {
                if let Some((x, y)) = find_xy_intersection(&rays[i], &rays[j]) {
                    let lo = lo as f64;
                    let hi = hi as f64;
                    if x >= lo && x <= hi && y >= lo && y <= hi {
                        count += 1;
                    }
                }
            }
        }
        count
    } else {
        // check all velocities (up to 1000), but start with small ones
        for d in 0..=1000 {
            for i in -d..=d {
                for j in -d..=d {
                    for (dx, dy, dz) in [
                        (d, i, j),
                        (-d, i, j),
                        (i, d, j),
                        (i, -d, j),
                        (i, j, d),
                        (i, j, -d),
                    ] {
                        let v = Vec3::new(dx, dy, dz);
                        if let Some(ts) = calculate_collision_times(&rays, v) {
                            let s = Ray::new(rays[0].evaulate(ts[0]), v).evaulate(-ts[0]);
                            verify_rays_converge(&rays, Ray::new(s, v), &ts); // just to be sure
                            return (s.x + s.y + s.z) as u64;
                        }
                    }
                }
            }
        }
        panic!("no match found");
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn xy_intersections_crossing() {
        let res = find_xy_intersection(
            &Ray::new(Vec3::new(19, 13, 30), Vec3::new(-2, 1, -2)),
            &Ray::new(Vec3::new(18, 19, 22), Vec3::new(-1, -1, -2)),
        )
        .unwrap();

        assert_approx_eq!(res.0, 14.333, 1e-3);
        assert_approx_eq!(res.1, 15.333, 1e-3);
    }

    #[test]
    fn xy_intersections_parallel() {
        assert_eq!(
            find_xy_intersection(
                &Ray::new(Vec3::new(18, 19, 22), Vec3::new(-1, -1, -2)),
                &Ray::new(Vec3::new(20, 25, 34), Vec3::new(-2, -2, -4)),
            ),
            None
        );
    }
}
