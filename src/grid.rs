use crate::{CELL_WIDTH, GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug)]
struct Cell {
    objects: [usize; Cell::MAX_CELL_SIZE],
    objects_count: usize,
}

impl Cell {
    const MAX_CELL_SIZE: usize = 8;
    const MAX_CELL_INDEX: usize = Cell::MAX_CELL_SIZE - 1;

    pub fn new() -> Self {
        Self {
            objects: [0; Cell::MAX_CELL_SIZE],
            objects_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.objects_count = 0;
    }

    pub fn add(&mut self, object: usize) {
        if self.objects_count == Cell::MAX_CELL_INDEX {
            return;
        }
        self.objects[self.objects_count] = object;
        self.objects_count += 1;
    }

    pub fn get_objects(&self) -> &[usize] {
        &self.objects[..self.objects_count]
    }
}

pub struct Grid {
    data: [Cell; GRID_WIDTH * GRID_HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        let arr: Vec<Cell> = (0..GRID_WIDTH * GRID_HEIGHT).map(|_| Cell::new()).collect();
        Self {
            data: arr.try_into().unwrap(),
        }
    }

    pub fn clear(&mut self) {
        for cell in self.data.iter_mut() {
            cell.clear();
        }
    }

    pub fn add_object(&mut self, x: f32, y: f32, object_id: usize) {
        let column_index = (x / CELL_WIDTH).floor() as usize;
        let row_index = (y / CELL_WIDTH).floor() as usize;
        if column_index >= GRID_WIDTH || row_index >= GRID_HEIGHT {
            return;
        }
        self.data[row_index * GRID_HEIGHT + column_index].add(object_id);
    }

    pub fn get_cell_objects(&self, row_index: usize, column_index: usize) -> &[usize] {
        self.data[column_index * GRID_HEIGHT + row_index].get_objects()
    }
}
