use crate::{
    application_controller::physics_controller, linear_algebra_math::scale, physics_math::*,
};

pub const CB_VERTEX_SHADER: &'static str = r#"
        #version 150

        in vec3 position;
        in vec3 normal;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model_matrix;

        out vec3 v_normal;

        void main() {
            v_normal = transpose(inverse(mat3(model_matrix))) * normal;
            gl_Position = perspective * view * model_matrix * vec4(position, 1.0);
        }
    "#;

pub const CB_FRAGMENT_SHADER: &'static str = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

pub struct CelestialBody {
    pub id: usize,
    pub radius: f64,
    pub mass: f64,
    pub velocity: [f64; 3],
    //color: [f64; 3],
    pub cartesian_position: [f64; 3],
    polar_position: [f64; 3],
}

impl CelestialBody {
    pub fn new(
        id: usize,
        radius: f64,
        mass: f64,
        velocity: [f64; 3],
        //color: [f64; 3],
        cartesian_position: [f64; 3],
        //angular_velocity: f64,
        //axis_of_rotation: f64,
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

    // apply a translation to the existing coords
    pub fn modify_position_polar(&mut self, rho: f64, theta: f64, phi: f64) {
        self.update_position_polar(
            (self.polar_position[0] + rho).abs(),
            (self.polar_position[1] + theta) % (2.0 * std::f64::consts::PI),
            (self.polar_position[2] + phi) % (2.0 * std::f64::consts::PI),
        );
    }

    // create new coords
    pub fn update_position_polar(&mut self, rho: f64, theta: f64, phi: f64) {
        self.polar_position = [rho, theta, phi];
        let (x, y, z) = polar_to_cartesian(rho, theta, phi);
        self.cartesian_position = [x, y, z];
    }

    pub fn modify_position_cartesian(&mut self, x: f64, y: f64, z: f64) {
        self.update_position_cartesian(
            self.cartesian_position[0] + x,
            self.cartesian_position[1] + y,
            self.cartesian_position[2] + z,
        );
    }

    pub fn update_position_cartesian(&mut self, x: f64, y: f64, z: f64) {
        self.cartesian_position = [x, y, z];
        let (rho, theta, phi) = cartesian_to_polar(x, y, z);
        self.polar_position = [rho, theta, phi];
    }

    // applies the stored velocity to the position
    pub fn apply_velocity(&mut self) {
        // need to scale with the per tick scalar so that the sim moves at the correct speed
        let time_scaled_velocity = scale(self.velocity, physics_controller::PER_TICK_SCALAR);
        self.modify_position_cartesian(
            time_scaled_velocity[0],
            time_scaled_velocity[1],
            time_scaled_velocity[2],
        );
    }

    pub fn set_velocity(&mut self, v1: [f64; 3]) {
        self.velocity = v1;
    }
}
