mod unit;
mod enemy_turn;
mod misc;
mod turn;
mod game;
use game::run_game;

fn main() { // runs the game 
    println!("Tri By Combat ");
    println!("--------------------------------");
    run_game();
}

