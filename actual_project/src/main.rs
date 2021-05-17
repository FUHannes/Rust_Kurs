pub mod chess_piece;
pub mod pawn;
use crate::chess_piece::ChessPiece;
use crate::pawn::Pawn;

fn main() {
    println!("Hello, world!");
    let x = Pawn;
    println!("{}",x.to_string());
}

