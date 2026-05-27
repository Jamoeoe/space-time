use crate::application_controller::celestial_body::CelestialBody;

pub const C: f32 = 299792458.0;
pub const G: f32 = 0.000000000066743015;
pub const K: f32 = 0.000000000000000000000000000000000000000000207665; // 8piG/c^4

pub fn cartesian_to_polar(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let rho = (x * x + y * y + z * z).sqrt();
    let theta = (z / rho).acos();
    let phi = y.signum() * (x / (x * x + y * y).sqrt()).acos();
    return (rho, theta, phi);
}

pub fn polar_to_cartesian(rho: f32, theta: f32, phi: f32) -> (f32, f32, f32) {
    let x = rho * theta.sin() * phi.cos();
    let y = rho * theta.sin() * phi.sin();
    let z = rho * theta.cos();
    return (x, y, z);
}

pub fn calculate_gravitational_pull(cb1: CelestialBody, cb2: CelestialBody) -> f32 {
    let force = G * cb1.mass * cb2.mass / distance_between_cbs_squared(cb1, cb2);
    return force;
}

pub fn distance_between_cbs(cb1: CelestialBody, cb2: CelestialBody) -> f32 {
    return distance_between_points(cb1.cartesian_position, cb2.cartesian_position);
}

pub fn distance_between_points(p1: [f32; 3], p2: [f32; 3]) -> f32 {
    let x_dst = (p1[0] - p2[0]) * (p1[0] - p2[0]);
    let y_dst = (p1[1] - p2[1]) * (p1[1] - p2[1]);
    let z_dst = (p1[2] - p2[2]) * (p1[2] - p2[2]);

    let dst = (x_dst + y_dst + z_dst).sqrt();
    return dst;
}

// common case needs distance squared and sqrt is a very expensive operation to
pub fn distance_between_cbs_squared(cb1: CelestialBody, cb2: CelestialBody) -> f32 {
    return distance_between_points_squared(cb1.cartesian_position, cb2.cartesian_position);
}

pub fn distance_between_points_squared(p1: [f32; 3], p2: [f32; 3]) -> f32 {
    let x_dst = (p1[0] - p2[0]) * (p1[0] - p2[0]);
    let y_dst = (p1[1] - p2[1]) * (p1[1] - p2[1]);
    let z_dst = (p1[2] - p2[2]) * (p1[2] - p2[2]);

    let dst_sqrd = x_dst + y_dst + z_dst;
    return dst_sqrd;
}
