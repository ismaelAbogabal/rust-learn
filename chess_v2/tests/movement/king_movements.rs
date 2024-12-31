use chess_v2::movement::movement::MovementTypes;

use super::check_movements::check_movements_of;

#[test]
fn king_starter_move() {
    let piece = 'k';
    let positions = vec![(0, 4), (7, 4)];
    let to_be = MovementTypes::King;

    check_movements_of(&piece, positions, to_be);
}

#[test]
fn king_movements() {
    let piece = 'k';
    let positions = vec![(1, 4), (6, 4)];
    let to_be = MovementTypes::Single;

    check_movements_of(&piece, positions, to_be);
}
