use crate::{
    linear_algebra_math::{add, cross3, normalize, subtract},
    physics_math::polar_to_cartesian,
};
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

pub fn make_sphere_triangles(precision: u32) -> (Vec<Vertex>, Vec<Normal>) {
    let (vertices, position_map) = calculate_sphere_vertices(precision);

    let vertex_triangle_normals_map = calculate_normals_of_triangles(&vertices, position_map);

    let vertex_normals: Vec<Normal> = calculate_vertex_normals(precision, vertex_triangle_normals_map);

    return (vertices, vertex_normals);
}

pub fn insert_triangle_indeces(dict: &mut HashMap<[u32; 2], Vec<u32>>, key: [u32; 2], index: u32) {
    match dict.get_mut(&key) {
        Some(v) => {
            v.push(index);
        }
        None => {
            dict.insert(key, vec![index]);
        }
    }
}

fn calculate_sphere_vertices(precision: u32) -> (Vec<Vertex>, HashMap<[u32; 2], Vec<u32>>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut position_map: HashMap<[u32; 2], Vec<u32>> = HashMap::new();

    for theta in 0..precision {
        for phi in 0..precision {
            let square_index = (theta * phi + phi) / 3;
            for square_half in 0..2 {
                let triangle_index = square_index + square_half;

                let relative_triangle_position = [
                    [theta, phi],
                    [theta + square_half, phi + 1 - square_half],
                    [theta + 1, phi + 1],
                ];

                for point in relative_triangle_position {
                    let (x, y, z) = polar_to_cartesian(
                        1.0,
                        2.0 * PI / precision as f64 * point[0] as f64,
                        PI / precision as f64 * point[1] as f64,
                    );
                    vertices.push(Vertex {
                        position: [x, y, z],
                    });

                    insert_triangle_indeces(&mut position_map, point, triangle_index);
                }
            }
        }
    }

    return (vertices, position_map);
}

fn calculate_normals_of_triangles(
    shape: &Vec<Vertex>,
    position_map: HashMap<[u32; 2], Vec<u32>>,
) -> HashMap<[u32; 2], [f64; 3]> {
    let mut normals: Vec<[f64; 3]> = vec![];

    for i in 0..(shape.len() / 3) {
        let normal = cross3(
            subtract(shape[i * 3 + 1].position, shape[i * 3].position),
            subtract(shape[i * 3 + 2].position, shape[i * 3].position),
        );

        normals.push(normalize(normal));
    }

    let mut vertex_normal_map: HashMap<[u32; 2], [f64; 3]> = HashMap::new();

    for (key, value) in position_map {
        let mut normals_sum = [0_f64; 3];

        for triangle_index in value {
            normals_sum = add(normals_sum, normals[triangle_index as usize])
        }

        vertex_normal_map.insert(key, normalize(normals_sum));
    }

    return vertex_normal_map;
}

fn calculate_vertex_normals(
    precision: u32,
    vertex_triangle_normal_map: HashMap<[u32; 2], [f64; 3]>,
) -> Vec<Normal> {
    let mut vertex_normals: Vec<Normal> = vec![];

    for theta in 0..precision {
        for phi in 0..precision {
            for square_half in 0..2 {
                let relative_triangle_position = [
                    [theta, phi],
                    [theta + square_half, phi + 1 - square_half],
                    [theta + 1, phi + 1],
                ];

                for point in relative_triangle_position {
                    match vertex_triangle_normal_map.get(&point) {
                        Some(&vertex_normal) => {
                            vertex_normals.push(Normal {
                                normal: vertex_normal,
                            });
                        }
                        None => {
                            panic!("Missing point when calculating normals for a sphere")
                        }
                    }
                }
            }
        }
    }
    return vertex_normals;
}
