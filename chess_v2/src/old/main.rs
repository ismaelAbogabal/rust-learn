mod board;
mod move_direction;
mod piece;

use board::Board;

// handle promote, pawn, king movements
// handle king checks
// evaluate position based on material and position
// travel the tree to dig depper
// evaluate the tree
// evaluate position based on tree
// convert back to string

fn main() {
    //todo
    let board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    println!("{}", board.to_terminal_string());
}
