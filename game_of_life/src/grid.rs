const CHANCE_TO_LIVE: f32 = 0.3;

const NEIGHBORS: [(i8, i8); 8] = [
    // top_left, top, top_right
    (-1, -1), (0, -1), (1, -1),
    // left, right
    (-1, 0), (1, 0),
    // bottom_left, bottom, bottom_right
    (-1, 1), (0, 1), (1, 1),
];

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn cells(&self) -> &[bool] {
        &self.cells
    }

    pub fn randomize(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = rand::random::<f32>() < CHANCE_TO_LIVE;
        }
    }

    pub fn next_gen(&mut self) {
        let mut next = vec![false; self.cells.len()];

        for (index, alive) in self.cells.iter().enumerate() {
            next[index] = match (alive, self.neighbors_alive(index)) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }

        self.cells = next;
    }

    fn neighbors_alive(&self, index: usize) -> u8 {
        let mut alive = 0;
        for (dx, dy) in NEIGHBORS {
            if self.cells[self.neighbor_index(index, dx, dy)] {
                alive += 1;
            }
        }
        alive
    }

    fn neighbor_index(&self, index: usize, dx: i8, dy: i8) -> usize {
        let width = self.width as isize;
        let size = width * self.height as isize;
        let (dx, dy) = (dx as isize, dy as isize);

        let column = index as isize % width;
        let row_start = index as isize - column;

        let new_column = (column + dx + width) % width;
        let new_row_start = (row_start + dy * width + size) % size;

        (new_row_start + new_column) as usize
    }
}
