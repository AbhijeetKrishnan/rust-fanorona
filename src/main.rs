use rand::prelude::SliceRandom;

fn main() {
    let mut board = rust_fanorona::Board::new();
    loop {
        println!("{}", board);
        let legal_moves = board.legal_moves();

        if legal_moves.is_empty() {
            println!("No legal moves available.");
            break;
        }
        let move_ = legal_moves
            .choose(&mut rand::thread_rng())
            .expect("Failed to choose a legal move");
        println!("{}", move_);
        let _ = board.push(*move_).expect("Failed to push move");
    }
    println!("{} wins!", board.winner().unwrap());
}
