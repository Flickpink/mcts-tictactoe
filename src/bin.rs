use tttlib::tictactoe::{GameState,Player};
use tttlib::mcts::{Arena, MCTS, Node};
use std::mem;



fn main() {
    // let mut arena: Arena<GameState> = Arena::default();

    println!("Memsize is: {}", mem::size_of::<Node<GameState>>());

    let mut gs = GameState::default();

    // loop {
    for _ in 0..2 {
        println!("{}",&gs);
        match gs.player {
            Player::Human => gs.exec_move(gs.rec_get_human_move()),
            Player::Computer => gs.exec_move(gs.get_mcts_move()),
            //Player::Computer => gs.exec_move(gs.get_rng_move()),
            Player::None => panic!(),
        }
    }
        println!("{}",&gs);

}
