mod game_of_life;

use game_of_life::GameOfLife;
use std::time::Instant;

const GRID_WIDTH: usize = 10000;
const GRID_HEIGHT: usize = 10000;
const STEPS: usize = 10;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let initial_grid = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);

    let mut game_seq: GameOfLife = initial_grid.clone();

    let start: Instant = Instant::now();
    for _ in 0..STEPS {
        game_seq.sequential_step();
    }
    let duration = start.elapsed();
    println!("Sequential: {:?}", duration);


    let mut game_par = initial_grid.clone();
    let start = Instant::now();
    for _ in 0..STEPS {
        game_par.parallel_step();
    }
    let duration = start.elapsed();
    println!("Parallel: {:?}", duration);

}
