use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    nrows: u32,
    ncols: u32,
    cells: Vec<bool>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(nrows: u32, ncols: u32) -> Self {
        Universe {
            nrows: nrows,
            ncols: ncols,
            cells: (0..nrows * ncols)
                .map(|i| i % 2 == 0 || i % 7 == 0)
                .collect(),
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let idx = self.get_index(row, col);
                let count = self.num_alive_neighbors(row, col);
                let is_alive = self.cells[idx];
                new_cells[idx] = count == 3 || (is_alive && count == 2)
            }
        }
        self.cells = new_cells;
    }

    pub fn cells(&self) -> *const bool {
        self.cells.as_ptr()
    }

    pub fn nrows(&self) -> u32 {
        self.nrows
    }

    pub fn ncols(&self) -> u32 {
        self.ncols
    }
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        ((self.ncols * row) + col) as usize
    }

    fn num_alive_neighbors(&self, row: u32, col: u32) -> u32 {
        let left = if col == 0 { self.ncols - 1 } else { col - 1 };
        let right = if col >= self.ncols - 1 { 0 } else { col + 1 };
        let top = if row == 0 { self.nrows - 1 } else { row - 1 };
        let bottom = if row >= self.nrows - 1 { 0 } else { row + 1 };

        [
            (top, left),
            (top, col),
            (top, right),
            (row, left),
            (row, right),
            (bottom, left),
            (bottom, col),
            (bottom, right),
        ]
        .iter()
        .map(|(i, j)| self.cells[self.get_index(*i, *j)] as u32)
        .sum()
    }
}
