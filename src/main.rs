extern crate cgmath;
extern crate gl;
extern crate glfw;

use std::mem::size_of;

use cgmath::*;
use cgmath::num_traits::ToPrimitive;
use gl::types::*;
use glfw::{Action, Context, Key};
use rand::{self, Rng};

use crate::engine::Engine;
use crate::resource_manager::ResourceManager;

mod colorgen;
mod engine;
mod grid;
mod grid_renderer;
mod particles_renderer;
mod resource_manager;
mod shader;
mod solver;
mod vertex;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 1200;
const RADIUS: f32 = 0.5;

const WORLD_SIZE: Vector2<f32> = cgmath::vec2(300.0, 300.0);

const CELL_WIDTH: f32 = RADIUS * 4.0;
const GRID_WIDTH: usize = (WORLD_SIZE.x / CELL_WIDTH) as usize;
const GRID_HEIGHT: usize = (WORLD_SIZE.y / CELL_WIDTH) as usize;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "OpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    let mut resource_manager = ResourceManager::new();
    resource_manager.load_shader("cell");
    resource_manager.load_shader("particle");

    let mut engine = Engine::new(&resource_manager);

    window.set_key_polling(true);
    window.set_framebuffer_size_callback(|_window, width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut vao: GLuint = 0;
    let mut vertex_vbo: GLuint = 0;
    let mut positions_vbo: GLuint = 0;

    let mut cells_vao: GLuint = 0;
    let mut cells_vbo: GLuint = 0;
    let mut cells: Vec<cgmath::Vector2<f32>> = Vec::with_capacity(GRID_WIDTH * GRID_HEIGHT);
    for x in (0..=GRID_HEIGHT) {
        for y in 0..=GRID_WIDTH {
            cells.push(cgmath::vec2(x as f32 * CELL_WIDTH, y as f32 * CELL_WIDTH));
        }
    }
    unsafe {
        gl::GenVertexArrays(1, &mut cells_vao);
        gl::GenBuffers(1, &mut cells_vbo);
        gl::BindVertexArray(cells_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, cells_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            cells.len() as isize * size_of::<cgmath::Vector2<f32>>() as GLsizeiptr,
            cells.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vector2<f32>>() as GLint,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vertex_vbo);
        gl::GenBuffers(1, &mut positions_vbo);
        gl::BindVertexArray(vao);

        // positions.
        gl::BindBuffer(gl::ARRAY_BUFFER, positions_vbo);
        let positions_data = &engine
            .solver
            .get_objects()
            .iter()
            .map(|y| y.position)
            .collect::<Vec<cgmath::Vector2<f32>>>();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            positions_data.len() as isize * size_of::<cgmath::Vector2<f32>>() as GLsizeiptr,
            positions_data.as_ptr().cast(),
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vector2<f32>>() as GLint,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::VertexAttribDivisor(0, 1);

        // Unbind VAO.
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Enable(gl::PROGRAM_POINT_SIZE);
    }

    // 60 fps
    let delta_time = 1.0 / 60.0;
    while !window.should_close() {
        engine.update(delta_time as f32);

        unsafe {
            // update positions buffer in video memory
            gl::BindBuffer(gl::ARRAY_BUFFER, positions_vbo);
            let positions_data = &engine
                .solver
                .get_objects()
                .iter()
                .map(|y| y.position)
                .collect::<Vec<_>>();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                positions_data.len() as isize * size_of::<cgmath::Vector2<f32>>() as GLsizeiptr,
                positions_data.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // update colors
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            let data_bytes = &engine
                .solver
                .get_objects()
                .iter()
                .map(|y| y.color)
                .collect::<Vec<_>>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                data_bytes.len() as isize * size_of::<cgmath::Vector3<f32>>() as GLsizeiptr,
                data_bytes.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            // radius attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<cgmath::Vector3<f32>>() as GLint,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::VertexAttribDivisor(1, 1);
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            engine.render();
        }

        window.swap_buffers();

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    engine.add_at_position(100.0, 100.0);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    engine.change_gravity(-100.0, 0.0);
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    engine.change_gravity(100.0, 0.0);
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    engine.change_gravity(0.0, 100.0);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    engine.change_gravity(-0.0, -100.0);
                }
                glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                    engine.change_gravity(-100.0, 0.0);
                }
                glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    engine.change_gravity(100.0, 0.0);
                }
                glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                    engine.change_gravity(0.0, 100.0);
                }
                glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                    engine.change_gravity(-0.0, -100.0);
                }
                glfw::WindowEvent::Key(Key::G, _, Action::Press, _) => {
                    engine.toggle_add_objects();
                }
                _ => {}
            }
        }
    }
}
