#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: Vec::from(grid),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[self.cols * row + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[self.cols * row + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];

        if row > 0 {
            if col > 0 {
                res.push((row - 1, col - 1));
            }

            res.push((row - 1, col));

            if col < self.cols - 1 {
                res.push((row - 1, col + 1))
            }
        }

        if col > 0 {
            res.push((row, col - 1))
        }

        if col < self.cols - 1 {
            res.push((row, col + 1))
        }

        if row < self.rows - 1 {
            if col > 0 {
                res.push((row + 1, col - 1))
            }

            res.push((row + 1, col));

            if col < self.cols - 1 {
                res.push((row + 1, col + 1))
            }
        }

        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
		let mut diff = vec![];
        for i in 0..self.grid.rows {
            for j in 0..self.grid.cols {
                let mut alive_count = 0;
                let neighbours = self.grid.neighbours(i, j);

                for (x, y) in neighbours {
                    if let Cell::Alive = self.grid.get(x, y) {
                        alive_count += 1;
                    }
                }

                match self.grid.get(i, j) {
                    Cell::Alive => {
                        if alive_count < 2 || alive_count > 3 {
							diff.push((i, j));
                        }
                    }
                    Cell::Dead => {
                        if alive_count == 3 {
							diff.push((i, j));
                        }
                    }
                }
            }
        }

		for (x, y) in diff {
			self.grid.set(match self.grid.get(x, y) {
			    Cell::Alive => Cell::Dead,
			    Cell::Dead => Cell::Alive,
			}, x, y);
		}
    }
}
