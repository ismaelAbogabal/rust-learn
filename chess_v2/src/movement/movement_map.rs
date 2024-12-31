//todo generate the movement map

use std::{collections::HashMap, vec};

use crate::piece::Piece;

use super::movement::MovementTypes;

pub fn movement_map() -> HashMap<char, HashMap<usize, Vec<(usize, MovementTypes)>>> {
    let mut map = HashMap::new();

    //todo
    let knight_jumps = vec![
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];
    let knight_joins = generate_movements(&knight_jumps, MovementTypes::Single);
    map.insert(Piece::Knight.to_char(), knight_joins.clone());
    map.insert(Piece::BlackKnight.to_char(), knight_joins);

    let bishop_jumps = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let bishop_joins = generate_movements(&bishop_jumps, MovementTypes::Continues);
    map.insert(Piece::Bishop.to_char(), bishop_joins.clone());
    map.insert(Piece::BlackBishop.to_char(), bishop_joins);

    let rook_jumps = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut rook_joins = generate_movements(&rook_jumps, MovementTypes::Continues);
    let corner_positions = vec![0, 7, 56, 63];
    update_relations(&mut rook_joins, corner_positions, MovementTypes::RookStart);
    map.insert(Piece::Rook.to_char(), rook_joins.clone());
    map.insert(Piece::BlackRook.to_char(), rook_joins);

    let queen_moves = rook_jumps
        .iter()
        .chain(bishop_jumps.iter())
        .copied()
        .collect();
    let queen_joins = generate_movements(&queen_moves, MovementTypes::Continues);
    map.insert(Piece::Queen.to_char(), queen_joins.clone());
    map.insert(Piece::BlackQueen.to_char(), queen_joins.clone());

    // king first move has spicial charactaristic
    let mut king_joins = generate_movements(&queen_moves, MovementTypes::Single);
    update_relations(&mut king_joins, vec![4, 60], MovementTypes::King);
    map.insert(Piece::King.to_char(), king_joins.clone());
    map.insert(Piece::BlackKing.to_char(), king_joins.clone());

    map
}

fn update_relations(
    list: &mut HashMap<usize, Vec<(usize, MovementTypes)>>,
    of: Vec<usize>,
    to: MovementTypes,
) {
    of.into_iter().for_each(|position| {
        let relations = list.get_mut(&position).unwrap();
        for re in relations {
            re.1 = to.clone();
        }
    });
}

fn generate_movements(
    movements: &Vec<(i32, i32)>,
    movement_type: MovementTypes,
) -> HashMap<usize, Vec<(usize, MovementTypes)>> {
    let mut out = HashMap::new();

    for c in 0..8 {
        for r in 0..8 {
            let current = r * 8 + c;

            let moves = relative_move(current, &movements, &movement_type);

            out.insert(current, moves);
        }
    }

    out
}

fn relative_move(
    position: usize,
    movements: &Vec<(i32, i32)>,
    move_type: &MovementTypes,
) -> Vec<(usize, MovementTypes)> {
    let mut output = vec![];
    let position = (position / 8, position % 8);

    for mov in movements {
        let new_position = (position.0 as i32 + mov.0, position.1 as i32 + mov.1);

        if new_position.0 >= 0 && new_position.0 < 8 && new_position.1 >= 0 && new_position.1 < 8 {
            output.push((
                (new_position.0 as usize) * 8 + new_position.1 as usize,
                move_type.clone(),
            ));
        }
    }

    output
}

fn pawn_moves(white: bool) -> HashMap<usize, Vec<(usize, MovementTypes)>> {
    let movements =
        generate_movements(&vec![(1, 0), (-1, 0)], MovementTypes::PawnMove(Piece::None));

    let mut out = HashMap::new();

    let march_moves = vec![(1, 0)];
    let jumb_moves = vec![(2, 0)];
    let eat_moves = vec![(1, 1), (1, -1)];

    for r in 1..8 {
        for c in 0..8 {
            let position = r * 8 + c;
            let mut moves = vec![];

            if r == 1 {
                let jumb_moves =
                    relative_move(position, &jumb_moves, &MovementTypes::PawnMove(Piece::None));

                moves.extend(jumb_moves);
            } else if r == 4 {
                let opasson_moves = relative_move(position, &eat_moves, &MovementTypes::Opason);
                moves.extend(opasson_moves);
            }

            let march_moves = relative_move(
                position,
                &march_moves,
                &MovementTypes::PawnMove(Piece::None),
            );
            moves.extend(march_moves);

            let eat_moves =
                relative_move(position, &eat_moves, &MovementTypes::PawnEat(Piece::None));
            moves.extend(eat_moves);

            out.insert(position, moves);
        }
    }

    out
}
