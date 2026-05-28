#[macro_use]
extern crate glium;
use glium::winit::event_loop::EventLoop;

use crate::{
    application_controller::{SimApplicationController, camera::Camera, celestial_body::*, shapes},
    physics_math::{distance_between_points_squared, get_circular_orbital_velocity_at_height},
};

pub mod application_controller;
pub mod linear_algebra_math;
pub mod physics_math;

fn main() {
    let event_loop;
    match EventLoop::builder().build() {
        Ok(result) => {
            println!("hello");
            event_loop = result;
        }
        Err(e) => {
            panic!("builder got sad :( {e}");
        }
    }

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Space time sim")
        .with_inner_size(1920, 1080)
        .build(&event_loop);

    let mut earth = CelestialBody::new(
        0,
        6.38 * 10.0_f64.powi(6),
        5.972 * 10.0_f64.powi(24),
        [0.0_f64, 0.0_f64, 0.0_f64],
        [0.0_f64, 0.0_f64, 2.0],
    );

    // create celestial bodies
    let mut moon = CelestialBody::new(
        1,
        1.73 * 10.0_f64.powi(6),
        7.346 * 10.0_f64.powi(22),
        [0.0_f64, 0.0_f64, 0.0_f64],
        [3.84400000 * 10.0_f64.powi(8), 0.0_f64, 0.0_f64],
    );

    let (earth_v, moon_v) = get_circular_orbital_velocity_at_height(&earth, &moon);

    earth.set_velocity(earth_v);
    moon.set_velocity(moon_v);

    let bodies = vec![earth, moon];

    let mut furthest_dst_from_origin = 0.0_f64;
    let origin = [0.0_f64, 0.0_f64, 0.0_f64];

    for body in bodies.iter() {
        let dst = distance_between_points_squared(&body.cartesian_position, &origin);
        if dst > furthest_dst_from_origin {
            furthest_dst_from_origin = dst;
        }
    }

    let camera = Camera::new(
        // starting position for the camera
        [0.0_f64, 30000000.0_f64, furthest_dst_from_origin.sqrt()],
        origin, // point at origin
        // fov
        std::f64::consts::PI / 4.0,
        // doesnt render anything past this point on the z axis
        furthest_dst_from_origin,
        // doesnt render anything closer than this point on the z axis
        0.1,
    );

    // create vertex buffer for the celestial bodies
    let cb_vertex_buffer = shapes::get_buffer(&display, &shapes::make_sphere_lines(20));

    let mut app = SimApplicationController::new(
        window,
        display,
        camera,
        bodies,
        cb_vertex_buffer,
        furthest_dst_from_origin.sqrt(),
    );

    match event_loop.run_app(&mut app) {
        Ok(_) => {}
        Err(e) => panic!("got sad while starting the app {e}"),
    }
}
