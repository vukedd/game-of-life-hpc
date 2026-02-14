mod game_of_life;

use game_of_life::GameOfLife;

fn main() {
    let mut game = GameOfLife::new(100, 100);
    game.sequential_step();
}
