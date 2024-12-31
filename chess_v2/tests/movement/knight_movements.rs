use chess_v2::movement::movement::MovementTypes;

use super::check_movements::check_movements_of;

#[test]
fn knight_movement() {
    let piece = 'n';
    let positions = vec![(1, 4), (6, 4)];
    let to_be = MovementTypes::Single;

    check_movements_of(&piece, positions, to_be);
}
