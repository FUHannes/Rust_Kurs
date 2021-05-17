use crate::ChessPiece;

pub struct Pawn;

impl ChessPiece for Pawn {
    fn to_string(self)->String{
        String::from("p")
    }
}