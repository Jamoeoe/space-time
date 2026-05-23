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
