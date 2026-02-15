mod game_of_life;

use game_of_life::GameOfLife;

fn main() {
    let mut game = GameOfLife::new(5, 5);
    game.sequential_step();
    game.parallel_step(2);
}
