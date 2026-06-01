use crate::{
    CelestialBody,
};

struct Node {
    data: CelestialBody,
    kids: [Option<Box<Node>>; 8]
}

impl Node {
    fn new(data: CelestialBody) -> Self {
        return Node{data: data, kids: [const { None }; 8]};
    }
}

pub struct Octree {
    root: Option<Node>,
}

impl Octree {
    pub fn new() -> Self {
        return Octree { root: None}
    }
}