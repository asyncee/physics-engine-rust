use cgmath::num_traits::ToPrimitive;
use cgmath::ortho;
use rand::{Rng, thread_rng};

use crate::colorgen::ColorGenerator;
use crate::grid_renderer::GridRenderer;
use crate::particles_renderer::ParticlesRenderer;
use crate::resource_manager::ResourceManager;
use crate::solver::Solver;
use crate::vertex::Vertex;
use crate::WORLD_SIZE;

pub struct Engine<'a> {
    pub resource_manager: &'a ResourceManager,
    color_generator: ColorGenerator,
    pub solver: Solver,
    grid_renderer: GridRenderer<'a>,
    particles_renderer: ParticlesRenderer<'a>,
    add_objects: bool,
}

impl<'a> Engine<'a> {
    pub fn new(resource_manager: &'a ResourceManager) -> Self {
        let objects = Vec::with_capacity(1000);
        let solver = Solver::new(objects);
        let color_generator = ColorGenerator::new();

        let grid_renderer = GridRenderer::new(resource_manager);
        let particles_renderer = ParticlesRenderer::new(resource_manager);
        Self {
            resource_manager,
            color_generator,
            solver,
            grid_renderer,
            particles_renderer,
            add_objects: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.solver.update(delta_time);

        if self.add_objects {
            for i in 0..10 {
                let mut vx = Vertex::new(
                    cgmath::vec2(
                        250.0 + i as f32 * 1.0 + thread_rng().gen_range(1.0..20.0) as f32,
                        250.0 + i as f32 * 2.0,
                    ),
                    self.color_generator.next_color(),
                );
                vx.accelerate(cgmath::vec2(10.0, 0.0));
                self.solver.add(vx);
            }
        }
    }

    pub fn toggle_add_objects(&mut self) {
        self.add_objects = !self.add_objects;
    }

    pub fn add_at_position(&mut self, x: f32, y: f32) {
        for i in (0..10) {
            let i = i.to_f32().unwrap();
            for j in (0..10) {
                let j = j.to_f32().unwrap();
                let vx = Vertex::new(
                    cgmath::vec2(x - 5.0 + i, y + 5.0 - j),
                    self.color_generator.next_color(),
                );
                self.solver.add(vx);
            }
        }
    }

    pub fn change_gravity(&mut self, x: f32, y: f32) {
        self.solver.change_gravity(x, y);
    }

    pub fn render(&self) {
        let projection = ortho(0.0, WORLD_SIZE.x, 0.0, WORLD_SIZE.y, -1.0, 1.0);
        // self.grid_renderer.render(projection);
        self.particles_renderer
            .render(projection, self.solver.get_objects());
    }
}
