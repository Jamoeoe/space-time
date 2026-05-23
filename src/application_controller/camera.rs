use crate::{linear_algebra_math::*, physics_math::*};
use glium::{Frame, Surface};

pub struct Camera {
    cartesian_position: [f32; 3],
    polar_position: [f32; 3],
    target: [f32; 3],
    fov: f32,
    zfar: f32,
    znear: f32,
}

impl Camera {
    pub fn new(
        cartesian_position: [f32; 3],
        target: [f32; 3], // point at origin
        fov: f32,
        zfar: f32,
        znear: f32,
    ) -> Camera {
        let (rho, theta, phi) = cartesian_to_polar(
            cartesian_position[0],
            cartesian_position[1],
            cartesian_position[2],
        );
        Camera {
            cartesian_position: cartesian_position,
            polar_position: [rho, theta, phi],
            target: target, // point at origin
            fov: fov,
            zfar: zfar,
            znear: znear,
        }
    }

    pub fn get_perspective(&self, target: &Frame) -> [[f32; 4]; 4] {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let f = 1.0 / (self.fov / 2.0).tan();

        return [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (self.zfar + self.znear) / (self.zfar - self.znear),
                1.0,
            ],
            [
                0.0,
                0.0,
                -(2.0 * self.zfar * self.znear) / (self.zfar - self.znear),
                0.0,
            ],
        ];
    }

    pub fn look_at(&mut self) -> [[f32; 4]; 4] {
        let camera_direction = normalize(subtract(self.target, self.cartesian_position));
        let up = [0.0, 0.0, 1.0f32];
        let right_axis = normalize(cross3(up, camera_direction));
        let up_axis = cross3(camera_direction, right_axis);

        return [
            [right_axis[0], up_axis[0], camera_direction[0], 0.0],
            [right_axis[1], up_axis[1], camera_direction[1], 0.0],
            [right_axis[2], up_axis[2], camera_direction[2], 0.0],
            [
                -dot(right_axis, self.cartesian_position),
                -dot(up_axis, self.cartesian_position),
                -dot(camera_direction, self.cartesian_position),
                1.0,
            ],
        ];
    }

    pub fn update_position_polar(&mut self, rho: f32, theta: f32, phi: f32) {
        self.polar_position = [rho, theta, phi];
        let (x, y, z) = polar_to_cartesian(rho, theta, phi);
        self.cartesian_position = [x, y, z];
    }

    pub fn modify_position_polar(&mut self, rho: f32, theta: f32, phi: f32) {
        self.update_position_polar(
            (self.polar_position[0] + rho).abs(),
            (self.polar_position[1] + theta) % (2.0 * std::f32::consts::PI),
            (self.polar_position[2] + phi) % (2.0 * std::f32::consts::PI),
        );
    }

    pub fn update_position_cartesian(&mut self, x: f32, y: f32, z: f32) {
        self.cartesian_position = [x, y, z];
        let (rho, theta, phi) = polar_to_cartesian(x, y, z);
        self.polar_position = [rho, theta, phi];
    }
}
