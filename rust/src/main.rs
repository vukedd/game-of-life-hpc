mod game_of_life;

use chrono::Local;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use game_of_life::GameOfLife;
use std::time::Instant;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;
const STEPS: usize = 10;

fn main() -> std::io::Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let initial_grid = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);

    let mut game_seq: GameOfLife = initial_grid.clone();

    let results_path = create_results_dir()?;
    println!("Saving results to: {}", results_path);

    let start: Instant = Instant::now();
    for _ in 0..STEPS {
        game_seq.sequential_step();
    }
    let duration = start.elapsed();
    println!("Sequential: {:?}", duration);


    let mut game_par = initial_grid.clone();
    save_grid(&game_par.grid, game_par.width, &results_path, 0)?;

    let start = Instant::now();
    for i in 0..STEPS {
        game_par.parallel_step();
        save_grid(&game_par.grid, game_par.width, &results_path, i + 1)?;
    }
    let duration = start.elapsed();
    println!("Parallel: {:?}", duration);

    Ok(())
}

pub fn create_results_dir() -> std::io::Result<String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let dir_path = format!("results/run_{}", timestamp);

    fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}


fn generate_file_name(step_no: usize) -> String {
    format!("gen_{:05}.csv", step_no)
}


pub fn save_grid(grid: &[u8], width: usize, dir_path: &str, step_no: usize) -> std::io::Result<()> {
    let file_name = generate_file_name(step_no);
    let file_path = Path::new(dir_path).join(file_name);

    let file = fs::File::create(file_path)?;

    let mut writer = BufWriter::new(file);

    for (i, &cell) in grid.iter().enumerate() {
        write!(writer, "{}", cell)?;

        let is_end_of_row = (i + 1) % width == 0;
        if is_end_of_row {
            writeln!(writer)?;
        } else {
            write!(writer, ",")?;
        }
    }

    Ok(())
}