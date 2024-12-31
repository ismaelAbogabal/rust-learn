#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,

    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
}

impl Piece {
    pub fn from_char(char: char) -> Piece {
        match char {
            'r' => Piece::Rook,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'q' => Piece::Queen,
            'k' => Piece::King,
            'p' => Piece::Pawn,
            'R' => Piece::BlackRook,
            'N' => Piece::BlackKnight,
            'B' => Piece::BlackBishop,
            'Q' => Piece::BlackQueen,
            'K' => Piece::BlackKing,
            'P' => Piece::BlackPawn,

            _ => Piece::None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Piece::None => '.',
            Piece::Pawn => 'p',
            Piece::Rook => 'r',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::King => 'k',
            Piece::Queen => 'q',
            Piece::BlackPawn => 'P',
            Piece::BlackRook => 'R',
            Piece::BlackKnight => 'N',
            Piece::BlackBishop => 'B',
            Piece::BlackKing => 'K',
            Piece::BlackQueen => 'Q',
        }
    }

    pub fn promote_to(white: bool) -> Vec<Piece> {
        if white {
            vec![Piece::Rook, Piece::Bishop, Piece::Knight, Piece::Queen]
        } else {
            vec![
                Piece::BlackRook,
                Piece::BlackBishop,
                Piece::BlackKnight,
                Piece::BlackQueen,
            ]
        }
    }

    pub fn is_white(&self) -> bool {
        match self {
            Piece::Pawn => true,
            Piece::Rook => true,
            Piece::Knight => true,
            Piece::Bishop => true,
            Piece::King => true,
            Piece::Queen => true,
            _ => false,
        }
    }
}
