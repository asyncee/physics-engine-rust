use cgmath::InnerSpace;

use crate::{CELL_WIDTH, GRID_HEIGHT, GRID_WIDTH, RADIUS, WORLD_SIZE};
use crate::grid::Grid;
use crate::vertex::Vertex;

pub struct Solver {
    gravity: cgmath::Vector2<f32>,
    objects: Vec<Vertex>,
    grid: Grid,
}

impl Solver {
    pub fn new(objects: Vec<Vertex>) -> Self {
        Self {
            gravity: cgmath::vec2(0.0, -1000.0),
            objects,
            grid: Grid::new(),
        }
    }

    pub fn add(&mut self, object: Vertex) {
        self.objects.push(object)
    }

    pub fn get_objects(&self) -> &Vec<Vertex> {
        return &self.objects;
    }

    pub fn update(&mut self, dt: f32) {
        let sub_steps = 8;
        let sub_dt = dt / sub_steps as f32;

        for _ in 0..sub_steps {
            // let start = Instant::now();
            self.apply_gravity(dt);
            // let gravity = start.elapsed();
            // let start = Instant::now();
            self.add_objects_to_grid();
            // let grid = start.elapsed();
            // let start = Instant::now();
            self.solve_collisions();
            // let collisions = start.elapsed();
            // let start = Instant::now();
            self.apply_constraints();
            // let constraints = start.elapsed();
            // let start = Instant::now();
            self.update_positions(sub_dt);
            // let positions = start.elapsed();
            // println!("gravity: {gravity:?}, constr: {constraints:?}, grid: {grid:?}, collisions: {collisions:?}, positions: {positions:?}");
        }
    }

    pub fn change_gravity(&mut self, x: f32, y: f32) {
        self.gravity = cgmath::vec2(x, y);
    }

    fn update_positions(&mut self, dt: f32) {
        for object in self.objects.iter_mut() {
            object.update_position(dt);
        }
    }

    fn apply_gravity(&mut self, dt: f32) {
        for object in self.objects.iter_mut() {
            object.accelerate(self.gravity);
        }
    }

    fn apply_constraints(&mut self) {
        // TODO: считать только по бокам границы.
        // if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
        for object in self.objects.iter_mut() {
            if object.position.x - RADIUS <= 0.0 {
                object.position.x = RADIUS;
            } else if object.position.x + RADIUS >= WORLD_SIZE.x {
                object.position.x = WORLD_SIZE.x - RADIUS;
            }
            if object.position.y - RADIUS <= 0.0 {
                object.position.y = 0.0 + RADIUS;
            } else if object.position.y + RADIUS >= WORLD_SIZE.y {
                object.position.y = WORLD_SIZE.y - RADIUS;
            }
        }
    }

    fn solve_collisions(&mut self) {
        for row in 0..GRID_WIDTH {
            for column in 0..GRID_HEIGHT {
                self.collide_cell(row, column);
            }
        }
    }

    fn add_objects_to_grid(&mut self) {
        self.grid.clear();
        for (i, object) in self.objects.iter().enumerate() {
            self.grid
                .add_object(object.position.x, object.position.y, i);
        }
    }

    fn collide_cell(&mut self, row: usize, column: usize) {
        for object_1_idx in self.grid.get_cell_objects(row, column) {
            for neighbour_row in -1..=1 {
                for neighbour_column in -1..=1 {
                    let row_idx = row as i32 - neighbour_row;
                    let col_idx = column as i32 - neighbour_column;
                    if row_idx < 0
                        || col_idx < 0
                        || row_idx >= GRID_WIDTH as i32
                        || col_idx >= GRID_HEIGHT as i32
                    {
                        continue;
                    }
                    Self::collide_nearby_cells(
                        &mut self.objects,
                        &self.grid,
                        *object_1_idx,
                        row_idx as usize,
                        col_idx as usize,
                    );
                }
            }
        }
    }

    fn collide_nearby_cells(
        objects: &mut Vec<Vertex>,
        grid: &Grid,
        object_1_idx: usize,
        row: usize,
        column: usize,
    ) {
        for object_2_idx in grid.get_cell_objects(row, column) {
            Self::collide_objects(objects, object_1_idx, *object_2_idx)
        }
    }

    fn collide_objects(objects: &mut Vec<Vertex>, object_1_idx: usize, object_2_idx: usize) {
        if object_1_idx == object_2_idx {
            return;
        }
        let lhs_pos = objects[object_1_idx].position;
        let rhs_pos = objects[object_2_idx].position;
        let collision_axis = lhs_pos - rhs_pos;
        let dist2 = collision_axis.magnitude2();
        if dist2 < CELL_WIDTH.powf(2.0) {
            let dist = dist2.sqrt();
            let normalized = collision_axis / dist;
            let delta = CELL_WIDTH - dist;
            let lhs = &mut objects[object_1_idx];
            lhs.position = lhs.position + 0.5 * normalized * delta;
            let rhs = &mut objects[object_2_idx];
            rhs.position = rhs.position - 0.5 * normalized * delta;
        }
    }
}
