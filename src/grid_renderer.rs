use std::mem::size_of;

use cgmath::{Matrix4, Vector2};
use gl::types::{GLint, GLsizeiptr, GLuint};

use crate::{CELL_WIDTH, GRID_HEIGHT, GRID_WIDTH};
use crate::resource_manager::ResourceManager;

pub struct GridRenderer<'a> {
    resource_manager: &'a ResourceManager,
    vao: GLuint,
    cells: Vec<Vector2<f32>>,
}

impl<'a> GridRenderer<'a> {
    pub fn new(resource_manager: &'a ResourceManager) -> Self {
        let mut renderer = Self {
            resource_manager,
            vao: 0,
            cells: vec![],
        };
        renderer.init_vao();
        renderer
    }

    fn init_vao(&mut self) {
        let mut vbo: GLuint = 0;
        for x in (0..=GRID_HEIGHT) {
            for y in 0..=GRID_WIDTH {
                self.cells.push(cgmath::vec2(
                    x as f32 * CELL_WIDTH as f32,
                    y as f32 * CELL_WIDTH as f32,
                ));
            }
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.cells.len() as isize * size_of::<cgmath::Vector2<f32>>() as GLsizeiptr,
                self.cells.as_ptr().cast(),
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
    }

    pub fn render(&self, projection: Matrix4<f32>) {
        unsafe {
            let cell_shader = self.resource_manager.get_shader("cell");
            cell_shader.use_shader();
            cell_shader.set_matrix4(projection, "projection");
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::POINTS, 0, self.cells.len() as i32);
        }
    }
}
