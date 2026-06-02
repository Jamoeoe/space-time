use crate::CelestialBody;

struct Node {
    data: CelestialBody,
    kids: [Option<Box<Node>>; 8],
    upper_box_bound: [f64; 3],
    lower_box_bound: [f64; 3],
}

impl Node {
    fn new(data: CelestialBody, upper_box_bound: [f64; 3], lower_box_bound: [f64; 3]) -> Self {
        return Node {
            data: data,
            kids: [const { None }; 8],
            upper_box_bound: upper_box_bound,
            lower_box_bound: lower_box_bound,
        };
    }
}

pub struct Octree {
    root: Option<Node>,
}

impl Octree {
    pub fn new() -> Self {
        return Octree { root: None };
    }
}

// returns true if within the box, false otherwise
fn check_coord_in_box(
    positions_to_check: [f64; 3],
    upper_box_bound: [f64; 3],
    lower_box_bound: [f64; 3],
) -> bool {
    if upper_box_bound[0] < positions_to_check[0] || positions_to_check[0] < lower_box_bound[0] {
        return false;
    }

    if upper_box_bound[1] < positions_to_check[1] || positions_to_check[1] < lower_box_bound[1] {
        return false;
    }

    if upper_box_bound[2] < positions_to_check[2] || positions_to_check[2] < lower_box_bound[2] {
        return false;
    }

    return true;
}
