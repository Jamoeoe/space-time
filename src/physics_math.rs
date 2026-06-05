use crate::{
    application_controller::celestial_body::CelestialBody,
    linear_algebra_math::{add, cross3, dot, length, normalize, rotate_vector, scale, subtract, unit_vector_between_vectors},
};

pub const C: f64 = 299792458.0;
pub const G: f64 = 0.000000000066743015;
pub const K: f64 = 0.000000000000000000000000000000000000000000207665; // 8piG/c^4

pub fn cartesian_to_polar(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let rho = (x * x + y * y + z * z).sqrt();
    let theta = (z / rho).acos();
    let phi = y.signum() * (x / (x * x + y * y).sqrt()).acos();
    return (rho, theta, phi);
}

pub fn polar_to_cartesian(rho: f64, theta: f64, phi: f64) -> (f64, f64, f64) {
    let x = rho * theta.sin() * phi.cos();
    let y = rho * theta.sin() * phi.sin();
    let z = rho * theta.cos();
    return (x, y, z);
}

pub fn calculate_gravitational_pull(cb1_mass: f64, cb2_mass: f64, distance_squared: f64) -> f64 {
    let force = G * cb1_mass * cb2_mass / distance_squared;
    return force;
}

pub fn distance_between_cbs(cb1: &CelestialBody, cb2: &CelestialBody) -> f64 {
    return distance_between_points(&cb1.cartesian_position, &cb2.cartesian_position);
}

pub fn distance_between_points(p1: &[f64; 3], p2: &[f64; 3]) -> f64 {
    let x_dst = (p1[0] - p2[0]) * (p1[0] - p2[0]);
    let y_dst = (p1[1] - p2[1]) * (p1[1] - p2[1]);
    let z_dst = (p1[2] - p2[2]) * (p1[2] - p2[2]);

    let dst = (x_dst + y_dst + z_dst).sqrt();
    return dst;
}

// common case needs distance squared and sqrt is a very expensive operation to compute
pub fn distance_between_cbs_squared(cb1: &CelestialBody, cb2: &CelestialBody) -> f64 {
    return distance_between_points_squared(&cb1.cartesian_position, &cb2.cartesian_position);
}

pub fn distance_between_points_squared(p1: &[f64; 3], p2: &[f64; 3]) -> f64 {
    let x_dst = (p1[0] - p2[0]) * (p1[0] - p2[0]);
    let y_dst = (p1[1] - p2[1]) * (p1[1] - p2[1]);
    let z_dst = (p1[2] - p2[2]) * (p1[2] - p2[2]);

    let dst_sqrd = x_dst + y_dst + z_dst;
    return dst_sqrd;
}

// gets the velocity needed to put cb2 into a circular orbit around cb1 at its height (assumes 2 body system)
pub fn get_circular_orbital_velocity_at_height(
    cb1: &CelestialBody,
    cb2: &CelestialBody,
) -> ([f64; 3], [f64; 3]) {
    let total_mass = cb1.mass + cb2.mass;

    // the force in newtons needed to orbit at the given distance
    let force_applied = (G * total_mass / distance_between_cbs(cb1, cb2)).sqrt();

    // the required velocity to orbit for each object
    let cb1_acceleration = force_applied * cb2.mass / total_mass;
    let cb2_acceleration = -force_applied * cb1.mass / total_mass;

    // direction perpendicular to the other mass
    let target_direction =
        unit_vector_between_vectors(cb1.cartesian_position, cb2.cartesian_position);

    let perenendicular_to_target_direction = cross3(target_direction, [0.0, 0.0, 1.0]);

    let cb1_velocity = scale(perenendicular_to_target_direction, cb1_acceleration);
    let cb2_velocity = scale(perenendicular_to_target_direction, cb2_acceleration);

    return (cb1_velocity, cb2_velocity);
}

// if they are colliding, returns the velocity impulse of the first body in the collision
pub fn check_collision(cb1: &CelestialBody, cb2: &CelestialBody, distance_squared: f64) -> (bool, [f64; 3]) {
    if distance_squared > (cb1.radius + cb2.radius) * (cb1.radius + cb2.radius) {
        return (false, [0.0; 3]);
    }

    // https://en.wikipedia.org/wiki/Elastic_collision
    let cb1_collision_impulse = add(scale(cb1.velocity, (cb1.mass - cb2.mass) / (cb1.mass + cb2.mass)), scale(cb2.velocity, 2.0 * cb2.mass / (cb1.mass + cb2.mass)));

    println!("{}, {:?}",cb1.id, cb1_collision_impulse);

    return (true, cb1_collision_impulse);
}