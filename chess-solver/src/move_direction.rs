// get the ultimate directions of the pices

// pawen has two spicial movements
// king has spicial movements (setelment)

use crate::board::Board;
use crate::piece::Piece;

impl Board {
    pub fn movements_of(&self, index: usize) -> Vec<Board> {
        // on the board calculate the piece movements

        //todo get the current pieces movement directions
        let piece = self.pieces[index];
        let white = piece.is_white();

        let directions = vec![] as Vec<Board>;
        if white ^ self.white_move || piece == Piece::None {
            return directions;
        }

        match piece {
            Piece::Pawn | Piece::BlackPawn => vec![],
            Piece::Queen | Piece::BlackQueen => vec![],
            Piece::Rook
            | Piece::Knight
            | Piece::Bishop
            | Piece::King
            | Piece::BlackRook
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackKing => self.pices_moves(index),
            _ => vec![],
        };

        directions
    }

    fn pawn_movements(&self, index: usize) -> Vec<Board> {
        let piece = self.pieces[index];

        if piece != Piece::Pawn && piece != Piece::BlackPawn {
            panic!("Invalid piece tring to move as pawn")
        }

        let mut out = vec![];
        let white = piece.is_white();
        let row = index / 8;
        let col = index % 8;

        let direction: i32 = if piece == Piece::BlackPawn { -1 } else { 1 };

        // can go forward
        let new_row = (row as i32 + direction) as usize;

        let new_index = new_row * 8 + col;

        if new_row != 0 && new_row != 7 {
            let new = self.generate_move(index, new_index);
            out.push(new);
        } else {
            let promote = Piece::promote(white);

            for item in promote {
                let mut board = self.clone();

                board.pieces[index] = Piece::None;
                board.pieces[new_index] = item;

                out.push(board);
            }
            //todo
            //promote
        }

        out
    }

    fn pices_moves(&self, index: usize) -> Vec<Board> {
        let piece = self.pieces[index];
        //
        let white = piece.is_white();
        let mut output = vec![] as Vec<Board>;
        let moves = piece.moves();
        let single_mover = piece.one_mover();

        for direction in moves {
            let mut starting = (index % 8, index / 8);

            loop {
                starting.0 += direction.0 as usize;
                starting.1 += direction.1 as usize;

                // get pieces on the new position

                let new_index = starting.0 + starting.1 * 8;
                let other = self.pieces[new_index];
                let other_white = other.is_white();

                if starting.0 >= 8 || starting.1 >= 8 {
                    break;
                } else if other == Piece::None {
                    output.push(self.generate_move(index, new_index));

                    // single movers can continue the loop
                    if single_mover {
                        break;
                    }
                    // can go forward
                } else if white ^ other_white {
                    output.push(self.generate_move(index, new_index));
                    break;
                } else {
                    //got to same pieces color block
                    break;
                }
            }
        }

        output
    }

    fn generate_move(&self, from: usize, to: usize) -> Board {
        //todo add new movement
        let mut new_board = self.clone();

        let piece = new_board.pieces[from];

        if piece == Piece::Rook {
        } else if piece == Piece::BlackRook {
        }

        new_board.pieces[to] = piece;
        new_board.pieces[from] = Piece::None;

        // if the moved piece is a pawn and it reaches the end of the board
        // check for passed pawn
        // check for rock moves to diable the gampet
        // check for king moves to diable the gampet

        new_board
    }
}

impl Piece {
    fn one_mover(&self) -> bool {
        match self {
            Piece::Pawn => true,
            Piece::BlackPawn => true,
            Piece::Knight => true,
            Piece::BlackKnight => true,
            Piece::BlackKing => true,
            Piece::King => true,
            Piece::Rook => false,
            Piece::Bishop => false,
            Piece::Queen => false,
            Piece::BlackRook => false,
            Piece::BlackBishop => false,
            Piece::BlackQueen => false,
            _ => false,
        }
    }

    fn moves(&self) -> Vec<(i8, i8)> {
        match self {
            Piece::None => vec![],
            Piece::Pawn => vec![],
            Piece::BlackPawn => vec![],
            Piece::Rook => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Piece::BlackRook => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Piece::Bishop => vec![(1, 1), (-1, 1), (1, -1), (-1, -1)],
            Piece::BlackBishop => vec![(1, 1), (-1, 1), (1, -1), (-1, -1)],
            Piece::King => vec![
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],
            Piece::Queen => vec![
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],

            Piece::Knight => vec![
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ],
            Piece::BlackKnight => vec![
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ],
            Piece::BlackKing => vec![
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],
            Piece::BlackQueen => vec![
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],
        }
    }

    fn is_white(&self) -> bool {
        match self {
            Piece::None => true,
            Piece::Pawn => true,
            Piece::Rook => true,
            Piece::Knight => true,
            Piece::Bishop => true,
            Piece::King => true,
            Piece::Queen => true,
            Piece::BlackPawn => false,
            Piece::BlackRook => false,
            Piece::BlackKnight => false,
            Piece::BlackBishop => false,
            Piece::BlackKing => false,
            Piece::BlackQueen => false,
        }
    }
}
