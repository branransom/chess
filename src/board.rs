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

impl PieceColor {}

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

impl PieceName {}

pub struct Board {
    board: [[Piece; 8]; 8],
    to_move: PieceColor,
}

impl Board {
    pub fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                let piece = match self.board[i][j] {
                    Piece {
                        name: PieceName::Pawn,
                        color: PieceColor::White,
                    } => "♙",
                    Piece {
                        name: PieceName::Knight,
                        color: PieceColor::White,
                    } => "♘",
                    Piece {
                        name: PieceName::Bishop,
                        color: PieceColor::White,
                    } => "♗",
                    Piece {
                        name: PieceName::Rook,
                        color: PieceColor::White,
                    } => "♖",
                    Piece {
                        name: PieceName::Queen,
                        color: PieceColor::White,
                    } => "♕",
                    Piece {
                        name: PieceName::King,
                        color: PieceColor::White,
                    } => "♔",
                    Piece {
                        name: PieceName::Pawn,
                        color: PieceColor::Black,
                    } => "♟︎",
                    Piece {
                        name: PieceName::Knight,
                        color: PieceColor::Black,
                    } => "♞",
                    Piece {
                        name: PieceName::Bishop,
                        color: PieceColor::Black,
                    } => "♝",
                    Piece {
                        name: PieceName::Rook,
                        color: PieceColor::Black,
                    } => "♜",
                    Piece {
                        name: PieceName::Queen,
                        color: PieceColor::Black,
                    } => "♛",
                    Piece {
                        name: PieceName::King,
                        color: PieceColor::Black,
                    } => "♚",
                    _ => ".",
                };
                print!("{}", piece)
            }
            println!();
        }
    }
}

pub fn new_board() -> Board {
    let mut b = [[Piece::create(PieceName::Pawn, PieceColor::White); 8]; 8];

    // White pieces
    b[0][0] = Piece::create(PieceName::Rook, PieceColor::White);
    b[0][1] = Piece::create(PieceName::Knight, PieceColor::White);
    b[0][2] = Piece::create(PieceName::Bishop, PieceColor::White);
    b[0][3] = Piece::create(PieceName::Queen, PieceColor::White);
    b[0][4] = Piece::create(PieceName::King, PieceColor::White);
    b[0][5] = Piece::create(PieceName::Bishop, PieceColor::White);
    b[0][6] = Piece::create(PieceName::Knight, PieceColor::White);
    b[0][7] = Piece::create(PieceName::Rook, PieceColor::White);

    for i in 0..8 {
        b[1][i] = Piece::create(PieceName::Pawn, PieceColor::White);
    }

    // No mans land
    for i in 2..6 {
        for j in 0..8 {
            b[i][j] = Piece::create(PieceName::Empty, PieceColor::Black);
        }
    }

    // Black pieces
    b[7][0] = Piece::create(PieceName::Rook, PieceColor::Black);
    b[7][1] = Piece::create(PieceName::Knight, PieceColor::Black);
    b[7][2] = Piece::create(PieceName::Bishop, PieceColor::Black);
    b[7][3] = Piece::create(PieceName::Queen, PieceColor::Black);
    b[7][4] = Piece::create(PieceName::King, PieceColor::Black);
    b[7][5] = Piece::create(PieceName::Bishop, PieceColor::Black);
    b[7][6] = Piece::create(PieceName::Knight, PieceColor::Black);
    b[7][7] = Piece::create(PieceName::Rook, PieceColor::Black);

    for i in 0..8 {
        b[6][i] = Piece::create(PieceName::Pawn, PieceColor::Black);
    }

    return Board {
        board: b,
        to_move: PieceColor::White,
    };
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
