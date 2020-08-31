extern crate pleco;
extern crate pleco_engine;

fn main() {
    use pleco::Board;
    use pleco_engine::{
        engine::PlecoSearcher,
        time::uci_timer::PreLimits,
    };

    let board_fen = "rnbqkbnr/nn6/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 100 50";
    let mut board = Board::from_fen(board_fen).unwrap();
    let mut limits = PreLimits::blank();
    limits.depth = Some(3);
    let mut searcher = PlecoSearcher::init(false);

    for i in 0..3 {
        searcher.search(&board, &limits);

        let mov = searcher.await_move();
        let from = mov.get_src();
        let to = mov.get_dest();

        println!("Move was from {} to {}", from.to_string(), to.to_string());
        board.apply_move(mov);
    }
}
