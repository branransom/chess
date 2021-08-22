mod board;

const STARTING_POSITION: [&str; 64] = [
    "r", "n", "b", "q", "k", "b", "n", "r", "p", "p", "p", "p", "p", "p", "p", "p", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "P", "P", "P", "P", "P", "P", "P", "P", "R",
    "N", "B", "Q", "K", "B", "N", "R",
];

const HORIZONTAL_SEPARATOR: &str = "+---";

fn build_horizontal_line() -> String {
    String::from("\n") + &HORIZONTAL_SEPARATOR.repeat(8) + "+"
}

fn build_vertical_line(row_num: u8) -> String {
    let mut row: String = String::from("\n");

    for i in 0..8 {
        let arr_pos: usize = (row_num * 8 + i).into();

        row = row + "| " + &STARTING_POSITION[arr_pos] + " ";
    }

    row + "|"
}

fn build_row(row_num: u8) -> String {
    build_horizontal_line() + &build_vertical_line(row_num)
}

fn build_board() -> String {
    let mut board: String = String::from("");

    for i in 0..8 {
        board = board + &build_row(i);
    }

    board + &build_horizontal_line()
}

fn chess_logo() -> &'static str {
    r#"
   ________
  / ____/ /_  ___  __________
 / /   / __ \/ _ \/ ___/ ___/
/ /___/ / / /  __(__  |__  )
\____/_/ /_/\___/____/____/
    "#
}

fn main() {
    println!("{}", chess_logo());
    println!("{}", build_board());
    board::Piece::create(board::PieceName::King, board::PieceColor::White);

    let my_board = board::new_board();
    my_board.print();
}
