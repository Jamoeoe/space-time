use crate::{application_controller::physics_controller, linear_algebra_math::scale, physics_math::*};

pub const CB_VERTEX_SHADER: &'static str = r#"
        #version 150

        in vec3 position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model_matrix;

        void main() {
            gl_Position = perspective * view * model_matrix * vec4(position, 1.0);
        }
    "#;

pub const CB_FRAGMENT_SHADER: &'static str = r#"
        #version 150

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

pub struct CelestialBody {
    pub id: usize,
    pub radius: f32,
    pub mass: f32,
    pub velocity: [f32; 3],
    //color: f32,
    pub cartesian_position: [f32; 3],
    polar_position: [f32; 3],
}

impl CelestialBody {
    pub fn new(
        id: usize,
        radius: f32,
        mass: f32,
        velocity: [f32; 3],
        //color: f32,
        cartesian_position: [f32; 3],
    ) -> CelestialBody {
        let (rho, theta, phi) = cartesian_to_polar(
            cartesian_position[0],
            cartesian_position[1],
            cartesian_position[2],
        );
        return CelestialBody {
            id: id,
            radius: radius,
            mass: mass,
            velocity: velocity,
            cartesian_position: cartesian_position,
            polar_position: [rho, theta, phi],
        };
    }

    pub fn modify_position_polar(&mut self, rho: f32, theta: f32, phi: f32) {
        self.update_position_polar(
            (self.polar_position[0] + rho).abs(),
            (self.polar_position[1] + theta) % (2.0 * std::f32::consts::PI),
            (self.polar_position[2] + phi) % (2.0 * std::f32::consts::PI),
        );
    }

    pub fn update_position_polar(&mut self, rho: f32, theta: f32, phi: f32) {
        self.polar_position = [rho, theta, phi];
        let (x, y, z) = polar_to_cartesian(rho, theta, phi);
        self.cartesian_position = [x, y, z];
    }

    pub fn modify_position_cartesian(&mut self, x: f32, y: f32, z: f32) {
        self.update_position_cartesian(
            self.cartesian_position[0] + x,
            self.cartesian_position[1] + y,
            self.cartesian_position[2] + z,
        );
    }

    pub fn update_position_cartesian(&mut self, x: f32, y: f32, z: f32) {
        self.cartesian_position = [x, y, z];
        let (rho, theta, phi) = polar_to_cartesian(x, y, z);
        self.polar_position = [rho, theta, phi];
    }

    pub fn apply_velocity(&mut self) {
        // need to scale with the per tick scalar so that the sim moves at the correct speed
        let time_scaled_velocity = scale(self.velocity, physics_controller::PER_TICK_SCALAR);
        self.modify_position_cartesian(time_scaled_velocity[0], time_scaled_velocity[1], time_scaled_velocity[2]);
    }

    pub fn set_velocity(&mut self, v1: [f32; 3]) {
        self.velocity = v1;
    }
}
