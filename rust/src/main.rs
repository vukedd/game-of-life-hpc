mod game_of_life;

use chrono::Local;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use game_of_life::GameOfLife;
use std::path::PathBuf;
use rayon::prelude::*;
use plotters::prelude::*;
// use std::time::Instant;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const STEPS: usize = 100;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let initial_grid = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);

    // let mut game_seq: GameOfLife = initial_grid.clone();

    let results_path = create_results_dir()?;
    println!("Saving results to: {}", results_path);

    // let start: Instant = Instant::now();
    // for _ in 0..STEPS {
    //     game_seq.sequential_step();
    // }
    // let duration = start.elapsed();
    // println!("Sequential: {:?}", duration);


    let mut game_par = initial_grid.clone();
    save_grid(&game_par.grid, game_par.width, &results_path, 0)?;

    // let start = Instant::now();
    for i in 0..STEPS {
        game_par.parallel_step();
        save_grid(&game_par.grid, game_par.width, &results_path, i + 1)?;
    }

    visualize_results(&results_path, GRID_WIDTH, GRID_HEIGHT)?;
    // let duration = start.elapsed();
    // println!("Parallel: {:?}", duration);

    Ok(())
}

// helper used to ensure the result directory is created
fn create_results_dir() -> std::io::Result<String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let dir_path = format!("results/run_{}", timestamp);

    fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}

// helper used to generate file name based on the iteration count
fn generate_file_name(step_no: usize) -> String {
    format!("gen_{:05}.csv", step_no)
}

// saves grid from memory to csv file as a grid
fn save_grid(grid: &[u8], width: usize, dir_path: &str, step_no: usize) -> std::io::Result<()> {
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

// used to visualize every generation csv file as a png file, 
fn visualize_results(results_path: &str, width: usize, height: usize) -> Result<(), Box<dyn std::error::Error>> {
    let results_dir = Path::new(results_path);
    
    // reads every csv file from the result directory and collects them into a Vector of string paths
    let mut csv_files: Vec<PathBuf> = fs::read_dir(results_dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "csv" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // makes sure files are sorted by generation so it can be presented properly
    csv_files.sort();

    if csv_files.is_empty() {
        println!("No CSV files found in {}", results_path);
        return Ok(());
    }

    println!("Found {} files. Starting parallel rendering...", csv_files.len());

    // using rayons par_iter function we process chunks of csv files and generate an image for every of them
    csv_files.par_iter().for_each(|path| {
        let mut png_path = path.clone();
        png_path.set_extension("png");

        match load_grid_from_csv(path) {
            Ok(grid) => {
                if let Err(e) = render_to_png(&grid, width, height, &png_path) {
                    eprintln!("Error rendering {:?}: {}", path, e);
                }
            }
            Err(e) => eprintln!("Error loading {:?}: {}", path, e),
        }
    });

    println!("Visualization complete. Images are located in: {}", results_path);
    Ok(())
}

// convert a csv grid into u8 grid for easier calculations
fn load_grid_from_csv(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut grid = Vec::new();

    for line in content.lines() {
        for val in line.split(',') {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                grid.push(trimmed.parse::<u8>()?);
            }
        }
    }
    Ok(grid)
}

// takes in-memory grid, it's dimensions and the output path, draws the generation output or returns an error
//
// we can't store trait objects directly on the stack because the size is unknown in compile time so we sent it to the
// heap, with dyn we say "hey return data of any type which implements the Error trait"
fn render_to_png(grid: &[u8], width: usize, height: usize, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // scale pixel size to 30, so the image is clearer
    let pixel_size = 30; 
    let img_width = (width * pixel_size) as u32;
    let img_height = (height * pixel_size) as u32;


    let root = BitMapBackend::new(output_path, (img_width, img_height)).into_drawing_area();
    
    root.fill(&WHITE)?;

    for (i, &cell) in grid.iter().enumerate() {
        if cell == 1 {
            let x = (i % width) as i32;
            let y = (i / width) as i32;

            let x0 = x * pixel_size as i32;
            let y0 = y * pixel_size as i32;
            let x1 = x0 + pixel_size as i32;
            let y1 = y0 + pixel_size as i32;

            root.draw(&Rectangle::new([(x0, y0), (x1, y1)], BLACK.filled()))?;
        }
    }

    root.present()?;
    Ok(())
}