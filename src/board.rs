use std::convert::TryInto;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub name: PieceName,
}

impl Piece {
    pub fn create(name: PieceName, color: PieceColor) -> Self {
        Self { name, color }
    }

    fn simple_char(&self) -> &'static str {
        match (self.color, self.name) {
            (PieceColor::Black, PieceName::Pawn) => "p",
            (PieceColor::Black, PieceName::Knight) => "n",
            (PieceColor::Black, PieceName::Bishop) => "b",
            (PieceColor::Black, PieceName::Rook) => "r",
            (PieceColor::Black, PieceName::Queen) => "q",
            (PieceColor::Black, PieceName::King) => "k",
            (PieceColor::White, PieceName::Pawn) => "P",
            (PieceColor::White, PieceName::Knight) => "N",
            (PieceColor::White, PieceName::Bishop) => "B",
            (PieceColor::White, PieceName::Rook) => "r",
            (PieceColor::White, PieceName::Queen) => "Q",
            (PieceColor::White, PieceName::King) => "K",
            _ => " ",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn opponent(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Tile {
    occupant: Piece,
}

impl Tile {
    pub fn new(occupant: Piece) -> Tile {
        Tile { occupant }
    }
}

pub struct Board {
    board: [[Tile; 8]; 8],
    to_move: PieceColor,
}

impl Board {
    pub fn new() -> Board {
        // 64 tiles
        let mut b = [[Tile::new(Piece::create(PieceName::Empty, PieceColor::White)); 8]; 8];

        // White pieces
        b[0][0] = Tile::new(Piece::create(PieceName::Rook, PieceColor::White));
        b[0][1] = Tile::new(Piece::create(PieceName::Knight, PieceColor::White));
        b[0][2] = Tile::new(Piece::create(PieceName::Bishop, PieceColor::White));
        b[0][3] = Tile::new(Piece::create(PieceName::Queen, PieceColor::White));
        b[0][4] = Tile::new(Piece::create(PieceName::King, PieceColor::White));
        b[0][5] = Tile::new(Piece::create(PieceName::Bishop, PieceColor::White));
        b[0][6] = Tile::new(Piece::create(PieceName::Knight, PieceColor::White));
        b[0][7] = Tile::new(Piece::create(PieceName::Rook, PieceColor::White));

        for i in 0..8 {
            b[1][i] = Tile::new(Piece::create(PieceName::Pawn, PieceColor::White));
        }

        // Black pieces
        for i in 0..8 {
            b[6][i] = Tile::new(Piece::create(PieceName::Pawn, PieceColor::Black));
        }

        b[7][0] = Tile::new(Piece::create(PieceName::Rook, PieceColor::Black));
        b[7][1] = Tile::new(Piece::create(PieceName::Knight, PieceColor::Black));
        b[7][2] = Tile::new(Piece::create(PieceName::Bishop, PieceColor::Black));
        b[7][3] = Tile::new(Piece::create(PieceName::Queen, PieceColor::Black));
        b[7][4] = Tile::new(Piece::create(PieceName::King, PieceColor::Black));
        b[7][5] = Tile::new(Piece::create(PieceName::Bishop, PieceColor::Black));
        b[7][6] = Tile::new(Piece::create(PieceName::Knight, PieceColor::Black));
        b[7][7] = Tile::new(Piece::create(PieceName::Rook, PieceColor::Black));

        return Board {
            board: b,
            to_move: PieceColor::White,
        };
    }

    // TODO: Convert u64 into tiles
    pub fn move_piece(&mut self, from: u8, to: u8) {
        let from_rank: usize = (from / 8).try_into().unwrap();
        let from_file: usize = (from % 8).try_into().unwrap();
        let to_rank: usize = (to / 8).try_into().unwrap();
        let to_file: usize = (to % 8).try_into().unwrap();

        self.board[to_rank][to_file] = self.board[from_rank][from_file];
        self.board[from_rank][from_file] =
            Tile::new(Piece::create(PieceName::Empty, PieceColor::White));
        self.to_move = self.to_move.opponent();
    }

    pub fn print(&self) {
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = match self.board[i][j].occupant {
                    Piece {
                        name: PieceName::Pawn,
                        color: PieceColor::Black,
                    } => "♙",
                    Piece {
                        name: PieceName::Knight,
                        color: PieceColor::Black,
                    } => "♘",
                    Piece {
                        name: PieceName::Bishop,
                        color: PieceColor::Black,
                    } => "♗",
                    Piece {
                        name: PieceName::Rook,
                        color: PieceColor::Black,
                    } => "♖",
                    Piece {
                        name: PieceName::Queen,
                        color: PieceColor::Black,
                    } => "♕",
                    Piece {
                        name: PieceName::King,
                        color: PieceColor::Black,
                    } => "♔",
                    Piece {
                        name: PieceName::Pawn,
                        color: PieceColor::White,
                    } => "♟︎",
                    Piece {
                        name: PieceName::Knight,
                        color: PieceColor::White,
                    } => "♞",
                    Piece {
                        name: PieceName::Bishop,
                        color: PieceColor::White,
                    } => "♝",
                    Piece {
                        name: PieceName::Rook,
                        color: PieceColor::White,
                    } => "♜",
                    Piece {
                        name: PieceName::Queen,
                        color: PieceColor::White,
                    } => "♛",
                    Piece {
                        name: PieceName::King,
                        color: PieceColor::White,
                    } => "♚",
                    _ => ".",
                };
                print!("{}", piece)
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pieces_recognized() {
        assert_eq!(
            Piece::create(PieceName::Bishop, PieceColor::White).color,
            PieceColor::White
        );
    }
}
