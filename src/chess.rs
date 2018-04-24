use adequate_math::*;
use pleco::{Board, BitMove};

use data::*;


pub fn piece_at(position: Vec2<i32>, pieces: &[Piece]) -> Option<usize> {
    let mut result = None;
    for (index, piece) in pieces.iter().enumerate() {
        if piece.position == position {
            result = Some(index);
        }
    }
    result
}


pub fn generate_fen(pieces: &[Piece], whos_turn: ChessColor) -> String {
    use std::fmt::Write;

    let mut buffer = String::with_capacity(128);
    let mut empty_stretch = 0;

    for y in 0..8 {
        for x in 0..8 {
            match piece_at(vec2(x, 7 - y).as_i32(), pieces) {
                Some(index) => {
                    use ChessColor::*;
                    use PieceType::*;

                    if empty_stretch > 0 {
                        write!(buffer, "{}", empty_stretch)
                            .unwrap();
                        empty_stretch = 0;
                    }

                    let piece = &pieces[index];
                    let ch =
                        match (piece.color, piece.piece_type) {
                            (White, Pawn) => "P",
                            (White, King) => "K",
                            (White, Queen) => "Q",
                            (White, Bishop) => "B",
                            (White, Rook) => "R",
                            (White, Knight) => "N",
                            (Black, Pawn) => "p",
                            (Black, King) => "k",
                            (Black, Queen) => "q",
                            (Black, Bishop) => "b",
                            (Black, Rook) => "r",
                            (Black, Knight) => "n",
                        };
                    buffer.push_str(ch);
                }
                None => {
                    empty_stretch += 1;
                }
            }
        }

        if empty_stretch > 0 {
            write!(buffer, "{}", empty_stretch).unwrap();
            empty_stretch = 0;
        }

        if y < 7 {
            buffer.push_str("/");
        }
    }

    match whos_turn {
        ChessColor::White => buffer.push_str(" w "),
        ChessColor::Black => buffer.push_str(" b "),
    }

    buffer.push_str("KQkq - 0 1");

    buffer
}


pub fn decide_move(pieces: &[Piece], whos_turn: ChessColor) -> BitMove {
    use pleco_engine::{
        engine::PlecoSearcher,
        time::uci_timer::PreLimits,
    };

    let fen = generate_fen(&pieces, whos_turn);
    let board = Board::from_fen(&fen).unwrap();

    let mut limits = PreLimits::blank();
    limits.depth = Some(3);
    let mut searcher = PlecoSearcher::init(false);

    searcher.search(&board, &limits);

    searcher.await_move()
}
