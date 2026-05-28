use crate::{
    Camera,
    application_controller::{
        celestial_body::{
            CelestialBody, {CB_FRAGMENT_SHADER, CB_VERTEX_SHADER},
        },
        physics_controller::PhysicsController,
    },
};
use glium::{
    Display, Program, Surface, VertexBuffer,
    index::NoIndices,
    winit::{
        application::ApplicationHandler,
        event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
        event_loop,
        window::Window,
    },
};
use std::{
    thread,
    time::{self, Duration, SystemTime},
};

pub mod camera;
pub mod celestial_body;
pub mod physics_controller;
pub mod shapes;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub const TARGET_FPS: f32 = 60.0;
const TARGET_FRAMETIME: Duration =
    Duration::new(0, (1.0 / TARGET_FPS * 1000000000.0).round() as u32);

pub struct SimApplicationController {
    window: Window,
    display: Display<glutin::surface::WindowSurface>,
    camera: Camera,
    physics_controller: PhysicsController,

    cb_vertex_buffer: VertexBuffer<Vertex>,
    cb_program: Program,
    cb_indices: NoIndices,

    scene_scale: f32,
    last_frame_time: SystemTime,

    // user controls
    mouse_dragging: bool,
    last_mouse_position: [f64; 2],
}

impl SimApplicationController {
    pub fn new(
        window: Window,
        display: Display<glutin::surface::WindowSurface>,
        camera: Camera,
        celestial_bodies: Vec<CelestialBody>,
        cb_vertex_buffer: glium::VertexBuffer<Vertex>,
        scene_scale: f32,
    ) -> SimApplicationController {
        let cb_indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);

        let cb_program;
        match glium::Program::from_source(&display, CB_VERTEX_SHADER, CB_FRAGMENT_SHADER, None) {
            Ok(p) => cb_program = p,
            Err(e) => panic!("celestial body program creation got sad :( {e}"),
        }

        return SimApplicationController {
            display: display,
            window: window,
            camera: camera,
            physics_controller: physics_controller::PhysicsController::new(celestial_bodies),

            cb_vertex_buffer: cb_vertex_buffer,
            cb_program: cb_program,
            cb_indices: cb_indices,

            scene_scale: scene_scale,
            last_frame_time: SystemTime::now(),

            mouse_dragging: false,
            last_mouse_position: [0.0; 2],
        };
    }
}

impl ApplicationHandler for SimApplicationController {
    fn resumed(&mut self, _event_loop: &event_loop::ActiveEventLoop) {}
    fn window_event(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("goodbye");
                std::process::exit(0)
            }

            WindowEvent::RedrawRequested => {
                self.physics_controller.tick();

                let time_since: Duration;
                match time::SystemTime::now().duration_since(self.last_frame_time) {
                    Ok(t) => time_since = t,
                    Err(_) => {
                        panic!("time moved backwards :(")
                    }
                }

                if time_since < TARGET_FRAMETIME {
                    thread::sleep(TARGET_FRAMETIME - time_since);
                }

                self.last_frame_time = SystemTime::now();

                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                let perspective = self.camera.get_perspective(&target);
                let view = self.camera.look_at();

                for celestial_body in &self.physics_controller.celestial_bodies {
                    // controls scale and position of each object
                    let model_matrix = [
                        [celestial_body.radius, 0.0, 0.0, 0.0],
                        [0.0, celestial_body.radius, 0.0, 0.0],
                        [0.0, 0.0, celestial_body.radius, 0.0],
                        [
                            celestial_body.cartesian_position[0],
                            celestial_body.cartesian_position[1],
                            celestial_body.cartesian_position[2],
                            1.0f32,
                        ],
                    ];

                    let uniforms = uniform! { perspective: perspective, view: view, model_matrix: model_matrix };

                    match target.draw(
                        &self.cb_vertex_buffer,
                        &self.cb_indices,
                        &self.cb_program,
                        &uniforms,
                        &Default::default(),
                    ) {
                        Ok(_) => {}
                        Err(e) => println!("got sad while drawing :( {e}"),
                    }
                }

                match target.finish() {
                    Ok(_) => {}
                    Err(e) => panic!("got sad while finishing :( {e}"),
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    if state == ElementState::Pressed {
                        self.mouse_dragging = true;
                    } else {
                        self.mouse_dragging = false;
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                let x_sensitivity = 0.0008;
                let y_sensitivity = 0.0006;

                if self.mouse_dragging {
                    let d_x = (self.last_mouse_position[0] - position.x) as f32;
                    let d_y = (self.last_mouse_position[1] - position.y) as f32;

                    self.camera.modify_position_polar(
                        0.0,
                        d_y * y_sensitivity,
                        -d_x * x_sensitivity,
                    );
                }

                self.last_mouse_position[0] = position.x;
                self.last_mouse_position[1] = position.y;
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_sensitivity = 0.05f32 * self.scene_scale;

                match delta {
                    MouseScrollDelta::LineDelta(_, d_rho) => {
                        self.camera
                            .modify_position_polar(-d_rho * zoom_sensitivity, 0.0, 0.0);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn about_to_wait(&mut self, _event_loop: &event_loop::ActiveEventLoop) {
        self.window.request_redraw();
    }
}
