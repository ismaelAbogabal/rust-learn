use chess_v2::movement::{movement::MovementTypes, movement_map::movement_map};

pub fn check_movements_of(piece: &char, positions: Vec<(usize, usize)>, to_be: MovementTypes) {
    let map = movement_map();

    let piece_moves = map.get(piece).expect("Cant find piece moves");

    for p in positions {
        let val = p.0 * 8 + p.1;
        //todo check that all moves are continuas
        let first_move = piece_moves.get(&val).expect("Missing the move 7");

        for m in first_move {
            let mt = &m.1;
            assert_eq!(mt, &to_be);
        }
    }
}
