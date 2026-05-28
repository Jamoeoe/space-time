use crate::{
    CelestialBody, application_controller,
    linear_algebra_math::{add, scale, unit_vector_between_vectors},
    physics_math::calculate_gravitational_pull,
};

pub const SIM_SPEED: f32 = 90000.0; // how fast the sim should move compared to realtime
pub const PER_TICK_SCALAR: f32 = SIM_SPEED / application_controller::TARGET_FPS;

pub struct PhysicsController {
    pub celestial_bodies: Vec<CelestialBody>,
}

impl PhysicsController {
    pub fn new(celestial_bodies: Vec<CelestialBody>) -> PhysicsController {
        return PhysicsController {
            celestial_bodies: celestial_bodies,
        };
    }

    pub fn tick(&mut self) {
        // calculate and apply gravity
        let mut gravity_impulses: Vec<[f32; 3]> = vec![];

        for (i, cb1) in self.celestial_bodies.iter().enumerate() {
            let mut cb_impulse: [f32; 3] = [0.0, 0.0, 0.0];

            for (j, cb2) in self.celestial_bodies.iter().enumerate() {
                if i != j {
                    let force = calculate_gravitational_pull(cb1, cb2);

                    let acceleration = force / cb1.mass;
                    let direction =
                        unit_vector_between_vectors(cb1.cartesian_position, cb2.cartesian_position);

                    cb_impulse = add(cb_impulse, scale(direction, -acceleration));
                }
            }

            gravity_impulses.push(cb_impulse);
        }

        for (i, cb) in self.celestial_bodies.iter_mut().enumerate() {
            let cb_impulse = gravity_impulses[i];

            let time_scaled_impulse = scale(cb_impulse, PER_TICK_SCALAR);
            cb.velocity = add(cb.velocity, time_scaled_impulse);

            if cb.id == 0 {
                println!("{:?}", cb.velocity);
            }

            cb.apply_velocity();
        }
    }
}
