#[macro_use]
extern crate glium;
use glium::winit::event_loop::EventLoop;

use crate::application_controller::{
    SimApplicationController, camera::Camera, celestial_body::*, shapes,
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

    let camera = Camera::new(
        // starting position for the camera
        [0.0, 2.0, 2.0],
        [0.0, 0.0, 0.0], // point at origin
        // fov
        std::f32::consts::PI / 4.0,
        // doesnt render anything past 1024z
        1024.0,
        // doesnt render anything closer than 0.1z
        0.1,
    );

    // create celestial bodies
    let sphere1 = CelestialBody::new(1.0, [0.0, 0.0, 0.0]);
    let sphere2 = CelestialBody::new(0.5, [2.0, 0.0, 2.0]);

    // create vertex buffer for the celestial bodies
    let cb_vertex_buffer = shapes::get_buffer(&display, &shapes::make_sphere_lines(20));

    let mut app = SimApplicationController::new(
        window,
        display,
        camera,
        vec![sphere1, sphere2],
        cb_vertex_buffer,
    );

    match event_loop.run_app(&mut app) {
        Ok(_) => {}
        Err(e) => panic!("got sad while starting the app {e}"),
    }
}
