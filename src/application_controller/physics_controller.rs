use crate::{
    CelestialBody, application_controller,
    linear_algebra_math::{add, scale, unit_vector_between_vectors},
    physics_math::{calculate_gravitational_pull, check_collision, distance_between_cbs_squared},
};

pub const SIM_SPEED: f64 = 2000000.0; // how fast the sim should move compared to realtime
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

        for cb1 in self.celestial_bodies.iter() {
            let mut cb_impulse: [f64; 3] = [0.0, 0.0, 0.0];

            for cb2 in self.celestial_bodies.iter() {
                    let distance_squared = distance_between_cbs_squared(cb1, cb2);
                    if distance_squared < 1.0 {
                            continue;
                    }

                    let (collided, cb1_angle, cb2_angle) = check_collision(cb1, cb2, distance_squared);

                    if collided {
                        let collision_energy = length(cb1.velocity) * cb1.mass + length(cb2.velocity) * cb2.mass;
                        // calculate unit vector at degree
                    }

                    let force = calculate_gravitational_pull(cb1.mass, cb2.mass, distance_squared);

                    let acceleration = force / cb1.mass;

                    let direction =
                        unit_vector_between_vectors(cb1.cartesian_position, cb2.cartesian_position);

                    cb_impulse = add(cb_impulse, scale(direction, -acceleration));
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
