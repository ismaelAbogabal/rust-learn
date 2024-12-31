use super::piece::Piece;

use super::castel::Castel;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub pieces: [Piece; 64],

    pub white_move: bool,
    // set to -1 to discard
    pub opason: usize,

    pub castels: Vec<Castel>, //todo handle the seatle
}

impl Board {
    pub fn empty() -> Self {
        return Self {
            pieces: [Piece::None; 64],
            white_move: true,
            opason: 8,
            castels: Castel::all(),
        };
    }

    pub fn from_str(str: &str) -> Self {
        let mut pieces = [Piece::None; 64];
        let mut index = 0;

        let a = str.split(' ');

        for char in str.chars() {
            //
            let piece = match char {
                '/' => continue,
                ' ' => break,
                char => Piece::from_char(char),
            };

            if piece != Piece::None {
                pieces[index] = piece;
                index += 1;
            } else {
                let num = char
                    .to_string()
                    .parse::<usize>()
                    .expect("Invalid character on the string");

                index += num;
            }
        }

        return Self {
            pieces,
            white_move: true,
            opason: 8,
            castels: Castel::all(),
        };
    }

    pub fn to_terminal_string(&self) -> String {
        let mut output = String::new();

        for i in 0..self.pieces.len() {
            // get the row 7 -> 0

            let row = 7 - i / 8;
            let col = i % 8;

            output.push(self.pieces[row * 8 + col].to_char());

            if col == 7 {
                output.push('\n');
            }
        }

        output
    }

    pub fn generate_move(&self, from: usize, to: usize) -> Board {
        let mut new_board = self.clone();
        new_board.opason = 8;

        new_board.pieces[to] = new_board.pieces[from];
        new_board.pieces[from] = Piece::None;

        new_board
    }
}
