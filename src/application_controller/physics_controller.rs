use crate::CelestialBody;

pub struct PhysicsController {
    pub celestial_bodies: Vec<CelestialBody>,
}

impl PhysicsController {
    pub fn new(celestial_bodies: Vec<CelestialBody>) -> PhysicsController {
        return PhysicsController {
            celestial_bodies: celestial_bodies,
        };
    }
}
