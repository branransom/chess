mod board;

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

    let my_board = board::Board::new();
    my_board.print();
}
