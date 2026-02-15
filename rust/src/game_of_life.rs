use rand::{thread_rng, Rng};
use rayon::prelude::*;


#[derive(Clone)]
pub struct GameOfLife {
    // 1D flat vectors are used to ensure the grid is a single contiguous block in memory. 
    // this maximizes "spatial locality," allowing the CPU to fetch a whole "cache line" 
    // (64 bytes) at once.
    //
    // in contrast, a Vec<Vec<bool>> is a collection of independent heap allocations. 
    // accessing a new row requires "pointer chasing," which often results in a cache miss 
    // as the CPU stalls while waiting for data to arrive from slow main RAM.
    //
    // flattening also enables the hardware prefetcher to predict future memory 
    // accesses and allows the compiler to use SIMD (Single Instruction, Multiple Data) 
    // to process multiple cells at once.
    //
    // working with u8 instead of boolean which we started with since it makes computation
    // more efficient and skips unneccesary operations such as if's and conditions and instead
    // uses simple addition
    pub grid: Vec<u8>, 
    scratch_grid: Vec<u8>,

    pub height: usize,
    pub width: usize,
}

impl GameOfLife {
    // new, initializes new random grid as well as a scratch grid which will store
    // the updates that occur on the original grid
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        
        let grid: Vec<u8> = (0..(width * height))
            .map(|_| if rng.gen_bool(0.5) { 1 } else { 0 })
            .collect();
            
        let scratch_grid = grid.clone();
        
        Self { grid, scratch_grid, height, width }
    }

    pub fn sequential_step(&mut self) {
        let width = self.width;
        let height = self.height;
        let grid = &self.grid;

        for y in 0..height {
            // calculates row index for cell which is logically above and wraps if it reached the beginning
            let y_up = if y == 0 { height - 1 } else { y - 1 };

            // calculates row index for cell which is logically below and wraps if it reached the end
            let y_down = if y == height - 1 { 0 } else { y + 1 };
            
            // calculate the exact position of cells we need the most to avoid doing it over and over again
            // up, curr, down to scan the state in which the neighbors are            
            let idx_up = y_up * width;
            let idx_curr = y * width;
            let idx_down = y_down * width;

            for x in 0..width {
                // for every cell of the row calculate it's left and right neighbors and wrap if they
                // are out of bounds
                let x_left = if x == 0 { width - 1 } else { x - 1 };
                let x_right = if x == width - 1 { 0 } else { x + 1 };
                
                // since values are u8's we can just do basic addition to find out the number of alive
                // neighbors
                let count = 
                    grid[idx_up + x_left]   + grid[idx_up + x]   + grid[idx_up + x_right] +
                    grid[idx_curr + x_left]                      + grid[idx_curr + x_right] +
                    grid[idx_down + x_left] + grid[idx_down + x] + grid[idx_down + x_right];

                let idx = idx_curr + x;
                let current_state = grid[idx];
                
                self.scratch_grid[idx] = if count == 3 || (count == 2 && current_state == 1) {
                    1
                } else {
                    0
                };
            }
        }
        std::mem::swap(&mut self.grid, &mut self.scratch_grid);
    }


    pub fn parallel_step(&mut self) {
        let width = self.width;
        let height = self.height;
        let grid = &self.grid;

        self.scratch_grid
            // splits the flat grid into chunks of size width and hands them out to different CPU cores
            // which will compute their new state concurrently
            .par_chunks_exact_mut(width)
            .enumerate()
            .for_each(|(y, row_slice)| {
                // calculates row index for cell which is logically above and wraps if it reached the beginning
                let y_up = if y == 0 { height - 1 } else { y - 1 };

                // calculates row index for cell which is logically below and wraps if it reached the end
                let y_down = if y == height - 1 { 0 } else { y + 1 };
                
                // calculate the exact position of cells we need the most to avoid doing it over and over again
                // up, curr, down to scan the state in which the neighbors are
                let idx_up = y_up * width;
                let idx_curr = y * width;
                let idx_down = y_down * width;

                for x in 0..width {
                    // for every cell of the row calculate it's left and right neighbors and wrap if they
                    // are out of bounds
                    let x_left = if x == 0 { width - 1 } else { x - 1 };
                    let x_right = if x == width - 1 { 0 } else { x + 1 };
                    
                    // since values are u8's we can just do basic addition to find out the number of alive
                    // neighbors
                    let count = 
                        grid[idx_up + x_left] + grid[idx_up + x] + grid[idx_up + x_right] +
                        grid[idx_curr + x_left]                    + grid[idx_curr + x_right] +
                        grid[idx_down + x_left] + grid[idx_down + x] + grid[idx_down + x_right];

                    let self_alive = grid[idx_curr + x];
                    
                    row_slice[x] = if count == 3 || (count == 2 && self_alive == 1) {
                        1
                    } else {
                        0
                    };
                }
            });

        // efficiently swap and avoid cloning
        std::mem::swap(&mut self.grid, &mut self.scratch_grid);
    }
}