use std::mem::size_of;

use cgmath::{Matrix4, Vector2, Vector3};
use gl::types::{GLint, GLsizeiptr, GLuint};

use crate::resource_manager::ResourceManager;
use crate::vertex::Vertex;

pub struct ParticlesRenderer<'a> {
    resource_manager: &'a ResourceManager,
    vao: GLuint,
    vbo: GLuint,
}

impl<'a> ParticlesRenderer<'a> {
    pub fn new(resource_manager: &'a ResourceManager) -> Self {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }
        let mut renderer = Self {
            resource_manager,
            vao,
            vbo,
        };
        renderer.init_vao();
        renderer
    }

    fn init_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            // positions.
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (size_of::<Vector2<f32>>() + size_of::<Vector3<f32>>()) as GLint,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribDivisor(0, 1);

            // colors
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (size_of::<(Vector2<f32>, Vector3<f32>)>()) as GLint,
                size_of::<Vector2<f32>>() as *const _,
            );
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::VertexAttribDivisor(1, 1);

            // Unbind VAO.
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&self, projection: Matrix4<f32>, particles: &Vec<Vertex>) {
        unsafe {
            let particles_buffer = particles
                .iter()
                .map(|y| (y.position, y.color))
                .collect::<Vec<_>>();

            // copy new positions and colors buffer in video memory - buffer already setup and configured.
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                particles_buffer.len() as isize
                    * size_of::<(Vector2<f32>, Vector3<f32>)>() as GLsizeiptr,
                particles_buffer.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            let particle_shader = self.resource_manager.get_shader("particle");
            particle_shader.use_shader();
            particle_shader.set_matrix4(projection, "projection");
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::POINTS, 0, 1, particles.len() as i32);
        }
    }
}
