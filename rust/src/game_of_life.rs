use std::sync::Arc;
use std::thread;
use rand::{thread_rng, Rng};

pub struct GameOfLife {
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();

        let grid = (0..height).map(|_| {
            (0..width).map(|_| rng.gen_bool(0.5)).collect()
        }).collect();
        
        Self { grid, height, width}
    }

    pub fn sequential_step(&mut self) {
        let mut new_grid = vec![vec![false; self.grid.len()]; self.grid[0].len()];
        let neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let alive_count: usize = neighbors.iter()
                    .filter(|(px, py)| {
                        let new_row = row as isize + px;
                        let new_col = col as isize + py;
                        new_row >= 0 && new_row < self.height as isize &&
                        new_col >= 0 && new_col < self.width as isize &&
                        self.grid[new_row as usize][new_col as usize]
                    }).count();
                            
                let is_alive: bool = (self.grid[row][col] && alive_count == 2) || alive_count == 3;
                new_grid[row][col] = is_alive;
            }
        }
        self.grid = new_grid;
    }

    pub fn parallel_step(&mut self, num_threads: usize) {
        println!("{:?}", self.grid);

        // define neighbor relative positions which will be used later
        let neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        
        // seting the size of the chunk which will be handled by a single thread
        let rows_per_thread = (self.height + num_threads - 1) / num_threads;

        // wrap grid, neighbors with Arc so it can be shared by multiple threads
        let grid_arc = Arc::new(self.grid.clone());
        let neighbors_arc = Arc::new(neighbors);

        // define height and width as new variables so we can use them in threads,
        // because if we use self.height/width in every thread the GOL. instance has
        // to be moved into multiple threads at once
        let height = self.height;
        let width = self.width;
        
        // create a range which size is determined on the number of threads specified
        //
        // handles is a vector of handles which represent references to a running/finished thread from
        // which we can extract the result
        let handles: Vec<thread::JoinHandle<(usize, Vec<Vec<bool>>)>> = (0..num_threads)
        // iterate and repeat the following logic for every thread
            .map(|thread_id: usize| {
                // create a new instance of the grid and neighbors Arc. They will
                // be used in separate threads
                let grid: Arc<Vec<Vec<bool>>> = Arc::clone(&grid_arc);
                let neighbors: Arc<[(isize, isize); 8]> = Arc::clone(&neighbors_arc);
                
                // starts a new thread where we compute the chunk, it is defined as a closure
                // so we can move variables to the thread 
                thread::spawn(move || {
                    let start_row: usize = thread_id * rows_per_thread;
                    let end_row: usize = ((thread_id + 1) * rows_per_thread).min(height);
                    
                    // initialize a fixed size vectorf to store the results of the current chunk and void
                    // vector expansion
                    let mut chunk_result: Vec<Vec<bool>> = Vec::with_capacity(end_row - start_row);
                    
                    for row in start_row..end_row {
                        let mut new_row: Vec<bool> = Vec::with_capacity(width);
                        
                        for col in 0..width {
                            let alive_count: usize = neighbors.iter()
                                .filter(|(px, py)| {
                                    let new_row = row as isize + px;
                                    let new_col = col as isize + py;
                                    new_row >= 0 && new_row < height as isize &&
                                    new_col >= 0 && new_col < width as isize &&
                                    grid[new_row as usize][new_col as usize]
                                }).count();
                            
                            let new_state: bool = (grid[row][col] && alive_count == 2) || alive_count == 3;
                            new_row.push(new_state);
                        }
                        
                        chunk_result.push(new_row);
                    }
                    
                    (start_row, chunk_result)
                })
            }).collect();

        let mut new_grid = vec![vec![false; self.width]; self.height];
        
        for handle in handles {
            // since there is almost no chance an error can occur here we don't have to pattern match
            // the result of handle.join() and it is okay to just unwrap it and stop the program if some
            // kind of error occurs
            let (start_row, chunk) = handle.join().unwrap();
            for (i, row) in chunk.into_iter().enumerate() {
                new_grid[start_row + i] = row;
            }
        }
        
        self.grid = new_grid;
        println!("{:?}", self.grid);
    }
}