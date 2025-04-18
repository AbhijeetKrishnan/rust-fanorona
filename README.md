# rust-fanorona

`rust-fanorona` is a Fanorona library for Rust, with move generation and move validation. It uses a novel format for representing the Fanorona board and moves.

This is an opening move in `rust-fanorona` -

```rust
use crate::rust_fanorona;

fn main() {
    let mut board = rust_fanorona::Board::new();
    println!("{}", board);
    let _ = board.push_str("e2n");
    println!("{}", board);
}
```

## Installation

`rust-fanorona` can be installed and used like any Rust crate. Simply add the following line to your `Cargo.toml` file -

```toml
[dependencies]
rust_fanorona = { git = "https://github.com/AbhijeetKrishnan/rust-fanorona.git", branch = "main" }
```

To view the documentation, use -

```bash
cargo doc -p rust-fanorona --open
```

## Notation

`rust-fanorona` uses a custom notation to represent the board state inspired by [FEN](https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt) from chess.

Moves are represented using the scheme described in the [ICGA article](https://icga.org/icga/games/Fanorona/#_Rules_of_Fanorona) on Fanorona. With the rows numbered from 1 to 5, and the columns indexed using letters `a` to `i`, a move `b2eb` indicates a move from b2 to c2 (east) with a withdrawal (backward) capture. For approach, the letter `F` (forward) is used instead.

The starting board state is represented as follows -

`WWWWWWWWW/WWWWWWWWW/BWBW1BWBW/BBBBBBBBB/BBBBBBBBB W - -`

After the move `e2nf`, the board state is -

`WWWWWWWWW/WWWWWWWWW/BWBW1BWBW/BBBB1BBBB/BBBB1BBBB W E2 E2NF`

The first field denotes the pieces at each location, with numbers used to indicate that number of consecutive empty spaces along the row. The next field denotes the turn. Then comes the (unordered) list of visited points during a capturing sequence, from first to last, and finally the last field indicates the last capturing move. These fields are `-` if not in a capturing sequence.