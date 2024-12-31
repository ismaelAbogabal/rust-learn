// we need to have relations between cells
// relation are different based piece type
// some movement might by invalid due to same color piece or it require oponent or like castel
// some movement might endup with multiple subboards like promote
// we need to checkout the checked status before doing it
//
//

use std::vec;

use crate::{board::Board, castel::Castel, piece::Piece};

pub struct BaseMove {
    // return the subboards and if it can move more
    move_it: fn(board: &Board, from: usize, to: usize) -> (Vec<Board>, bool),
}

trait Movement {
    fn get_moves(&self, board: &Board, from: usize, to: usize) -> (Vec<Board>, bool);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MovementTypes {
    // continue until find enemy/friend
    Continues,

    //disable the castel
    RookStart,

    // single move
    Single,

    Castel,
    King,

    PawnMove(Piece),
    PawnEat(Piece),
    Opason,
}

impl Movement for MovementTypes {
    fn get_moves(&self, board: &Board, from: usize, to: usize) -> (Vec<Board>, bool) {
        match &self {
            MovementTypes::Continues => simple_move(&board, from, to),
            MovementTypes::RookStart => rook_movement(board, from, to),
            MovementTypes::Single => {
                let mut simple = simple_move(board, from, to);
                simple.1 = false;
                simple
            }
            MovementTypes::PawnMove(change_to) => pawn_move(board, from, to, change_to),
            MovementTypes::PawnEat(change_to) => pawn_eat(board, from, to, change_to),
            MovementTypes::Opason => pawn_opason(board, from, to),
            MovementTypes::Castel => king_castel(board, from, to),
            // first king move otherwise it will single move
            MovementTypes::King => king_move(board, from, to),
        }
    }
}

fn king_move(board: &Board, from: usize, to: usize) -> (Vec<Board>, bool) {
    //
    let white = board.pieces[from].is_white();

    //
    let (mut board, _) = simple_move(board, from, to);

    for board in &mut board {
        for i in (0..board.castels.len()).rev() {
            let castel = board.castels[i];
            if (castel == Castel::BK || castel == Castel::BQ) ^ !white {
                board.castels.swap_remove(i);
            }
        }
    }

    (board, false)
}

fn rook_movement(board: &Board, from: usize, to: usize) -> (Vec<Board>, bool) {
    let mut movement = simple_move(board, from, to);

    let castle = Castel::from_rook(from);

    for sub in &mut movement.0 {
        let len = sub.castels.len();
        for i in 0..len {
            if sub.castels[i] == castle {
                sub.castels.swap_remove(i);
                break;
            }
        }
    }

    movement
}

fn king_castel(board: &Board, from: usize, to: usize) -> (Vec<Board>, bool) {
    let castel_option = Castel::from_positions(from, to);

    let mut can_castel = board.castels.contains(&castel_option);

    let rook = castel_option.rook_position();
    // check empty space between king and rook
    if can_castel {
        let range = if from > rook {
            rook + 1..from
        } else {
            from + 1..rook
        };

        for i in range {
            let piece = board.pieces[i];

            if piece != Piece::None {
                can_castel = false;
                break;
            }
        }
    }

    if !can_castel {
        (vec![], false)
    } else {
        let new_board = board.generate_move(from, to);

        let difference = (from + to) / 2;
        let new_board = new_board.generate_move(rook, difference);

        (vec![new_board], false)
    }
}

fn pawn_opason(board: &Board, from: usize, to: usize) -> (Vec<Board>, bool) {
    // todo
    let col = to % 8;
    let white_direction = to > from;

    let white = board.pieces[from].is_white();

    if white ^ white_direction {
        (vec![], false)
    } else if board.opason == col {
        let mut moved = board.generate_move(from, to);

        //todo remove passed pawn

        let pawn_index = if white { to - 8 } else { to + 8 };
        moved.pieces[pawn_index] = Piece::None;

        (vec![moved], false)
    } else {
        (vec![], false)
    }
}

fn pawn_eat(board: &Board, from: usize, to: usize, change_to: &Piece) -> (Vec<Board>, bool) {
    let pawn = board.pieces[from];
    let white = pawn.is_white();

    let oponent = board.pieces[to];

    let oponent_white = oponent.is_white();

    if white ^ oponent_white {
        let mut new_board = board.generate_move(from, to);
        if change_to != &Piece::None {
            new_board.pieces[to] = change_to.clone();
        }
        return (vec![new_board], false);
    } else {
        return (vec![], false);
    }
}

fn pawn_move(board: &Board, from: usize, to: usize, change_to: &Piece) -> (Vec<Board>, bool) {
    //
    let pawn = board.pieces[from];
    let white = pawn.is_white();

    let piece = board.pieces[to];

    if piece != Piece::None || white ^ (from < to) {
        return (vec![], false);
    }

    let mut new_board = board.generate_move(from, to);
    if change_to != &Piece::None {
        new_board.pieces[to] = change_to.clone();
    }

    return (vec![new_board], false);
}

fn simple_move(board: &Board, from: usize, to: usize) -> (Vec<Board>, bool) {
    let next_piece = board.pieces[to];
    let other_white = next_piece.is_white();

    let current_white = board.pieces[from].is_white();

    if next_piece == Piece::None {
        let new_board = board.generate_move(from, to);

        return (vec![new_board], true);
    } else if other_white ^ current_white {
        let new_board = board.generate_move(from, to);

        return (vec![new_board], false);
    } else {
        return (vec![], false);
    }
}
