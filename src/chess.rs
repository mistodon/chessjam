use okmath::*;
use pleco::{BitMove, Board};

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
                        write!(buffer, "{}", empty_stretch).unwrap();
                        empty_stretch = 0;
                    }

                    let piece = &pieces[index];
                    let ch = match (piece.color, piece.piece_type) {
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
    use pleco_engine::{engine::PlecoSearcher, time::uci_timer::PreLimits};

    let fen = generate_fen(&pieces, whos_turn);
    let board = Board::from_fen(&fen).unwrap();

    let mut limits = PreLimits::blank();
    limits.depth = Some(3);
    let mut searcher = PlecoSearcher::init(false);

    searcher.search(&board, &limits);

    searcher.await_move()
}


pub fn piece_price(piece_type: PieceType) -> &'static PiecePrice {
    const PAWN: PiecePrice = PiecePrice {
        buy_price: 4,
        discount_price: 3,
        sell_price: 2,
        unmoved_sell_price: 4,
    };
    const KNIGHT: PiecePrice = PiecePrice {
        buy_price: 5,
        discount_price: 3,
        sell_price: 3,
        unmoved_sell_price: 3,
    };
    const ROOK: PiecePrice = PiecePrice {
        buy_price: 6,
        discount_price: 4,
        sell_price: 3,
        unmoved_sell_price: 3,
    };
    const BISHOP: PiecePrice = PiecePrice {
        buy_price: 7,
        discount_price: 5,
        sell_price: 4,
        unmoved_sell_price: 4,
    };
    const QUEEN: PiecePrice = PiecePrice {
        buy_price: 9,
        discount_price: 6,
        sell_price: 5,
        unmoved_sell_price: 5,
    };

    match piece_type {
        PieceType::Pawn => &PAWN,
        PieceType::Knight => &KNIGHT,
        PieceType::Rook => &ROOK,
        PieceType::Bishop => &BISHOP,
        PieceType::Queen => &QUEEN,
        PieceType::King => unreachable!("Do not buy or sell kings!"),
    }
}

pub fn sell_price(piece_type: PieceType, moved: bool) -> u32 {
    let price = piece_price(piece_type);
    if moved {
        price.sell_price
    }
    else {
        price.unmoved_sell_price
    }
}

pub fn buy_price(piece_for_sale: PieceForSale) -> u32 {
    let price = piece_price(piece_for_sale.piece_type);
    if piece_for_sale.discounted {
        price.discount_price
    }
    else {
        price.buy_price
    }
}

pub fn valid_purchase_placements(
    pieces: &[Piece],
    piece_type: PieceType,
    color: ChessColor,
) -> Vec<Vec2<i32>> {
    let mut valid_purchase_placements = Vec::with_capacity(8);

    let y = match (color, piece_type) {
        (ChessColor::White, PieceType::Pawn) => 1,
        (ChessColor::White, _) => 0,
        (ChessColor::Black, PieceType::Pawn) => 6,
        (ChessColor::Black, _) => 7,
    };

    for x in 0..8 {
        let pos = vec2(x, y);
        if piece_at(pos, pieces).is_none() {
            valid_purchase_placements.push(pos);
        }
    }

    valid_purchase_placements
}
