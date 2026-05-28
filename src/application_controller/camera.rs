use crate::{linear_algebra_math::*, physics_math::*};
use glium::{Frame, Surface};

pub struct Camera {
    cartesian_position: [f64; 3],
    polar_position: [f64; 3],
    target: [f64; 3],
    fov: f64,
    zfar: f64,
    znear: f64,
}

impl Camera {
    pub fn new(
        cartesian_position: [f64; 3],
        target: [f64; 3],
        fov: f64,
        zfar: f64,
        znear: f64,
    ) -> Camera {
        let (rho, theta, phi) = cartesian_to_polar(
            cartesian_position[0],
            cartesian_position[1],
            cartesian_position[2],
        );
        Camera {
            cartesian_position: cartesian_position,
            polar_position: [rho, theta, phi],
            target: target,
            fov: fov,
            zfar: zfar,
            znear: znear,
        }
    }

    pub fn get_perspective(&self, target: &Frame) -> [[f32; 4]; 4] {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f64 / width as f64;
        let f = 1.0 / (self.fov / 2.0).tan();

        return [
            [f  as f32 * aspect_ratio  as f32, 0.0, 0.0, 0.0],
            [0.0, f  as f32, 0.0, 0.0],
            [
                0.0,
                0.0,
                ((self.zfar + self.znear) / (self.zfar - self.znear)) as f32,
                1.0,
            ],
            [
                0.0,
                0.0,
                (-(2.0 * self.zfar * self.znear) / (self.zfar - self.znear)) as f32,
                0.0,
            ],
        ];
    }

    pub fn look_at(&self) -> [[f32; 4]; 4] {
        let camera_direction = normalize(subtract(self.target, self.cartesian_position));
        let up = [0.0, 0.0, 1.0f64];
        let right_axis = normalize(cross3(up, camera_direction));
        let up_axis = cross3(camera_direction, right_axis);

        return [
            [right_axis[0] as f32, up_axis[0] as f32, camera_direction[0] as f32, 0.0],
            [right_axis[1] as f32, up_axis[1] as f32, camera_direction[1] as f32, 0.0],
            [right_axis[2] as f32, up_axis[2] as f32, camera_direction[2] as f32, 0.0],
            [
                -dot(right_axis, self.cartesian_position) as f32,
                -dot(up_axis, self.cartesian_position) as f32,
                -dot(camera_direction, self.cartesian_position) as f32,
                1.0,
            ],
        ];
    }

    pub fn set_target(&mut self, coords: [f64; 3]) {
        self.target = coords;
    }

    pub fn update_position_polar(&mut self, rho: f64, theta: f64, phi: f64) {
        self.polar_position = [rho, theta, phi];
        let (x, y, z) = polar_to_cartesian(rho, theta, phi);
        self.cartesian_position = [x, y, z];
    }

    pub fn modify_position_polar(&mut self, rho: f64, theta: f64, phi: f64) {
        self.update_position_polar(
            self.polar_position[0] + rho,
            (self.polar_position[1] + theta) % (2.0 * std::f64::consts::PI),
            (self.polar_position[2] + phi) % (2.0 * std::f64::consts::PI),
        );
    }

    pub fn update_position_cartesian(&mut self, x: f64, y: f64, z: f64) {
        self.cartesian_position = [x, y, z];
        let (rho, theta, phi) = polar_to_cartesian(x, y, z);
        self.polar_position = [rho, theta, phi];
    }
}
