fn main() {
    let mut board = rust_fanorona::Board::new();
    println!("{}", board);
    let _ = board.push_str("E2N");
    println!("{}", board);
}
