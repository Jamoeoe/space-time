use crate::{application_controller::Vertex, physics_math::polar_to_cartesian};
use glium::{Display, VertexBuffer};
use glutin::surface::WindowSurface;
use std::f64::consts::PI;

pub fn get_buffer(display: &Display<WindowSurface>, vertices: &[Vertex]) -> VertexBuffer<Vertex> {
    match glium::VertexBuffer::new(display, vertices) {
        Ok(buff) => buff,
        Err(e) => panic!("vertex buffer got sad :( {e}"),
    }
}

// makes a unit sphere out of lines
pub fn make_sphere_lines(precision: u32) -> Vec<Vertex> {
    //sphere
    let mut shape = vec![];

    for theta in 0..precision + 1 {
        for phi in 0..precision + 1 {
            let (x, y, z) = polar_to_cartesian(
                1.0,
                PI / precision as f64 * theta as f64,
                2.0 * PI / precision as f64 * phi as f64,
            );
            shape.push(Vertex {
                position: [x, y, z],
            });
        }
    }
    for phi in 0..precision {
        for theta in 0..precision + 1 {
            let (x, y, z) = polar_to_cartesian(
                1.0,
                PI / precision as f64 * theta as f64,
                2.0 * PI / precision as f64 * phi as f64,
            );

            shape.push(Vertex {
                position: [x, y, z],
            });
        }
    }

    return shape;
}

pub fn make_sphere_triangles(precision: i32) -> Vec<Vertex> {
    let mut shape = vec![];

    for theta in 0..precision {
        for phi in 0..precision {
            for half in 0..2 {
                let (x, y, z) = polar_to_cartesian(
                    1.0,
                    2.0 * PI / precision as f64 * (theta) as f64,
                    PI / precision as f64 * phi as f64,
                );
                shape.push(Vertex {
                    position: [x, y, z],
                });

                let (x, y, z) = polar_to_cartesian(
                    1.0,
                    2.0 * PI / precision as f64 * (theta + half) as f64,
                    PI / precision as f64 * (phi + 1 - half) as f64,
                );
                shape.push(Vertex {
                    position: [x, y, z],
                });

                let (x, y, z) = polar_to_cartesian(
                    1.0,
                    2.0 * PI / precision as f64 * (theta + 1) as f64 as f64,
                    PI / precision as f64 * (phi + 1) as f64,
                );
                shape.push(Vertex {
                    position: [x, y, z],
                });
            }
        }
    }

    return shape;
}

pub fn calculate_normals_of_triangles(vertices: Vec<Vertex>) -> Vec<Vertex> {
    let normals = vec![];

    return normals;
}
