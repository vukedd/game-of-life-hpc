pub struct GameOfLife {
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![true; width]; height];
        
        println!("{:?}", grid);
        Self { grid, height, width}
    }

    pub fn sequential_step(&mut self) {
        let mut new_grid = vec![vec![false; self.grid.len()]; self.grid[0].len()];
        let neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let mut alive_count: u8 = 0;

                for (px, py) in neighbors {
                    let new_row = row as isize + px;
                    let new_col: isize = col as isize + py;

                    if new_row < 0 || new_row >= self.height as isize ||
                       new_col < 0 || new_col >= self.width as isize {
                        continue;
                    }

                    if self.grid[new_row as usize][new_col as usize] {
                        alive_count += 1;
                    }
                }

                if (self.grid[row][col] && alive_count == 2) || alive_count == 3 {
                    new_grid[row][col] = true;
                } else {
                    new_grid[row][col] = false;
                }
            }
        }
        self.grid = new_grid;
        println!("{:?}", self.grid)
    }
}