use crate::{
    CelestialBody, application_controller,
    linear_algebra_math::{add, scale, unit_vector_between_vectors},
    physics_math::calculate_gravitational_pull,
};

struct Node {
    data: CelestialBody,
    kids: Vec<Option<Node>>
}

impl Node {
    fn new(data: CelestialBody, num_kids: u32) -> Self {
        let mut kids = vec![];
        for _ in 0..num_kids {
            kids.push(None);
        }
        return Node{data: data, kids: kids};
    }
}

pub struct Tree {
    root: Option<Node>,
}

impl Tree {
    pub fn new(_expected_elements: u64) -> Self {
        return Tree { root: None}
    }
}