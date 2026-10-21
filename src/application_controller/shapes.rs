use crate::physics_math::polar_to_cartesian;
use core::panic;
use glium::{Display, VertexBuffer};
use glutin::surface::WindowSurface;
use std::{collections::HashMap, f64::consts::PI, vec};

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f64; 3],
}
implement_vertex!(Vertex, position);

#[derive(Clone, Copy)]
pub struct Normal {
    pub normal: [f64; 3],
}
implement_vertex!(Normal, normal);

pub fn get_buffer<T: glium::Vertex>(
    display: &Display<WindowSurface>,
    vertices: &[T],
) -> VertexBuffer<T> {
    match glium::VertexBuffer::new(display, vertices) {
        Ok(buff) => buff,
        Err(e) => panic!("vertex buffer got sad :( {e}"),
    }
}

//fn calculate_sphere_vertices(precision: u32) -> (Vec<Vertex>, HashMap<[u32; 2], Vec<u32>>) {
pub fn calculate_sphere_vertices(precision: u32) -> (Vec<Vertex>, Vec<Normal>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut normals: Vec<Normal> = vec![];

    for theta in 0..precision {
        for phi in 0..precision {
            let relative_triangle_position = [[theta, phi], [theta, phi + 1], [theta + 1, phi + 1]];

            for point in relative_triangle_position {
                let (x, y, z) = polar_to_cartesian(
                    1.0,
                    PI / precision as f64 * point[0] as f64,
                    2.0 * PI / precision as f64 * point[1] as f64,
                );
                vertices.push(Vertex { position: [x, y, z] });
                normals.push(Normal { normal: [x, y, z] });
            }

            let relative_triangle_position = [[theta, phi], [theta + 1, phi + 1], [theta + 1, phi]];

            for point in relative_triangle_position {
                let (x, y, z) = polar_to_cartesian(
                    1.0,
                    PI / precision as f64 * point[0] as f64,
                    2.0 * PI / precision as f64 * point[1] as f64,
                );
                vertices.push(Vertex { position: [x, y, z] });
                normals.push(Normal { normal: [x, y, z] });
            }
        }
    }

    return (vertices, normals);
}
