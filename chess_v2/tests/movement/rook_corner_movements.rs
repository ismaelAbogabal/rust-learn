use std::vec;

use chess_v2::{movement::movement::MovementTypes, piece::Piece};

use super::check_movements::check_movements_of;

#[test]
pub fn rook_corner_movements() {
    let piece = Piece::Rook;

    let positions = vec![(0, 0), (0, 7), (7, 0), (7, 7)];
    let to_be = MovementTypes::RookStart;

    check_movements_of(&piece.to_char(), positions, to_be);
}

#[test]
pub fn rook_movments() {
    let piece = Piece::Rook;

    let positions = vec![(1, 0), (1, 7), (6, 0), (6, 7)];
    let to_be = MovementTypes::Continues;

    check_movements_of(&piece.to_char(), positions, to_be);
}
