use crate::{
    CelestialBody, application_controller,
    linear_algebra_math::{add, scale, unit_vector_between_vectors},
    physics_math::calculate_gravitational_pull,
};

pub const SIM_SPEED: f64 = 90000.0; // how fast the sim should move compared to realtime
pub const PER_TICK_SCALAR: f64 = SIM_SPEED / application_controller::TARGET_FPS;

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
        // create the list of impulses so that they can be edited after the nested for loop (because borrow checker)
        let mut gravity_impulses: Vec<[f64; 3]> = vec![];

        // for each celestial body
        for (i, cb1) in self.celestial_bodies.iter().enumerate() {
            let mut cb_impulse: [f64; 3] = [0.0, 0.0, 0.0];

            // for each celestial body that could affect the celestial body
            for (j, cb2) in self.celestial_bodies.iter().enumerate() {
                if i != j {
                    // calculate the total force in newtons
                    let force = calculate_gravitational_pull(cb1, cb2);

                    // acceleration is energy / mass
                    let acceleration = force / cb1.mass;

                    // apply the acceleration to the direction of the force to make an acceleration vector
                    let direction =
                        unit_vector_between_vectors(cb1.cartesian_position, cb2.cartesian_position);

                    // add the acceleration vector to all other sources of acceleration acting upon the celestial body
                    cb_impulse = add(cb_impulse, scale(direction, -acceleration));
                }
            }

            // save the overall impulse
            gravity_impulses.push(cb_impulse);
        }

        // apply the impulse to each celestial body
        for (i, cb) in self.celestial_bodies.iter_mut().enumerate() {
            let cb_impulse = gravity_impulses[i];

            // the rate at which things accelerate should scale with the speed of the sim
            let time_scaled_impulse = scale(cb_impulse, PER_TICK_SCALAR);
            cb.velocity = add(cb.velocity, time_scaled_impulse);

            cb.apply_velocity();
        }
    }
}
