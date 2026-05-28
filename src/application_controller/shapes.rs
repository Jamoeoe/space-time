use crate::{application_controller::Vertex, physics_math::polar_to_cartesian};
use glium::{Display, VertexBuffer};
use glutin::surface::WindowSurface;

pub fn get_buffer(display: &Display<WindowSurface>, vertices: &[Vertex]) -> VertexBuffer<Vertex> {
    match glium::VertexBuffer::new(display, vertices) {
        Ok(buff) => buff,
        Err(e) => panic!("vertex buffer got sad :( {e}"),
    }
}

// makes a unit sphere out of lines
pub fn make_sphere_lines(precision: i32) -> Vec<Vertex> {
    //sphere
    let mut shape = vec![];

    for theta in 0..precision + 1 {
        let theta_step = std::f64::consts::PI / precision as f64 * theta as f64;
        for phi in 0..precision + 1 {
            let phi_step = 2.0 * std::f64::consts::PI / precision as f64 * phi as f64;

            let (x, y, z) = polar_to_cartesian(1.0, theta_step, phi_step);
            shape.push(Vertex {
                position: [x, y, z],
            });
        }
    }
    for phi in 0..precision {
        let phi_step = 2.0 * std::f64::consts::PI / precision as f64 * phi as f64;
        for theta in 0..precision + 1 {
            let theta_step = std::f64::consts::PI / precision as f64 * theta as f64;

            let (x, y, z) = polar_to_cartesian(1.0, theta_step, phi_step);
            shape.push(Vertex {
                position: [x, y, z],
            });
        }
    }

    return shape;
}

/* pub fn make_sphere_triangles(precision: i32) -> Vec<Vertex> {

} */
