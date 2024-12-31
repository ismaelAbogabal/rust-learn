#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Castel {
    WK,
    WQ,
    BK,
    BQ,
}

impl Castel {
    pub fn all() -> Vec<Castel> {
        vec![Castel::WQ, Castel::WK, Castel::BK, Castel::BQ]
    }

    pub fn rook_position(&self) -> usize {
        match &self {
            Castel::WK => 7,
            Castel::WQ => 0,
            Castel::BK => 63,
            Castel::BQ => 56,
        }
    }

    // TODO: this is todo commit please fix it
    pub fn from_rook(from: usize) -> Self {
        match from {
            7 => Castel::WK,
            0 => Castel::WQ,
            63 => Castel::BK,
            56 => Castel::BQ,
            _ => panic!("Invalid rook"),
        }
    }

    pub fn from_positions(from: usize, to: usize) -> Self {
        // first line is made for white
        let white = from < 8;

        if to > from {
            if white {
                Castel::WK
            } else {
                Castel::BK
            }
        } else {
            if white {
                Castel::WQ
            } else {
                Castel::BQ
            }
        }
    }
}
