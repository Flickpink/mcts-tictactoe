use std::io;
use rand::{seq::IteratorRandom, thread_rng};
use std::fmt;

use crate::mcts::{Arena,MCTS};

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Player {
    None,
    Human,
    Computer,
}


// Board looks for entering commands:
// 1 2 3 
// 4 5 6
// 7 8 9

fn new_board() -> Vec<Player> {
    let mut v = Vec::with_capacity(9);
    for _ in 0..9 {
        v.push(Player::None)
    }
    v
}

fn parse_input() -> Option<usize> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");

    match input.trim().parse::<usize>() {
        Ok(idx) => match idx {
            1..=9 => Some(idx-1),
            _ => {
                println!("Please enter a single digit between 1 and 9.");
                None
            },
        }
        Err(_) => {
            println!("Please enter a single digit between 1 and 9.");
            None
        }
    }
}

fn rec_get_input() -> Option<usize> {
    match parse_input() {
        Some(x) => Some(x),
        None => rec_get_input(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub player: Player,
    pub board: Vec<Player>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player: Player::Human,
            board: new_board(),
        }
    }
}



// impl AvailableMoves for GameState {
    
//     fn available_moves(&self) -> Vec<GameState> {
//         for m in self.legal_moves() {
//             // TO DO
//         }
//     }

// }

impl GameState {

    // pub fn from(gs: &GameState) -> GameState {
    //     gs.clone()
    // }

    pub fn legal_moves(&self) -> Vec<usize> {
        self.board.iter().enumerate()
            .filter(|(_,m)| **m == Player::None)
            .map(|(i,_)|i)
            .collect()
    }

    // Because of the recursion in rec_get_input() this can always be unwrapped
    // safely.
    fn is_legal_move(&self, cmd: Option<usize>) -> Option<usize> {
        if self.legal_moves().contains(&cmd.unwrap()) {
            return cmd
        }
            println!("Field {:?} already taken!", &cmd.unwrap());
            None
    }

    pub fn rec_get_human_move(&self) -> Option<usize> {
        match self.is_legal_move(rec_get_input()) {
            Some(x) => Some(x),
            None => self.rec_get_human_move(),
        }
    }

    pub fn get_mcts_move(&self) -> Option<usize> {

        let mut arena = Arena::new_with_gamestate(self.clone());
    
        arena.iterate(1000);
        
        Some(*arena[0].children.iter().max_by_key(|child|arena.ucb1_of(**child)).unwrap())
    }
    
    pub fn get_rng_move(&self) -> Option<usize> {
        let mut rng = thread_rng();
        self.legal_moves().into_iter().choose(&mut rng)
    }


    // Returns None if game is not terminal, otherwise early return is triggered
    // 1 => Computer won | 0 => draw | -1 => Human won
    pub fn exec_mcts_move(&mut self, cmd: Option<usize>) -> Option<i8> {
        
        if self.available_moves().len() == 0 {
            return Some(0)
        }

        match cmd {
            Some(idx) => self.board[idx] = self.player,
            _ => panic!(),
        }

        if self.is_terminal() {
            match self.player {
                Player::Human => {
                    dbg!("Human won in this sim.");
                    return Some(-1);
                },
                Player::Computer => {
                    dbg!("Computer won in this sim.");
                    return Some(1);  
                },
                _ => panic!(),
            };
        }

        // Set next player
        self.player = match self.player {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
            _ => panic!(),
        };

        None

    }

    // Somewhat of a hack: The computer always has the last move. It samples
    // from an empty vector and returns 'None' if no move is availabe and we
    // terminate on that.
    pub fn exec_move(&mut self, cmd: Option<usize>) {
        
        // Make move
        match cmd {
            Some(idx) => self.board[idx] = self.player,
            None => {
                println!("Game over!");
                std::process::exit(0)
            }
        }

        if self.is_terminal() {
            match self.player {
                Player::Human => {
                    println!("{} \n You won human, this cannot be!", self);
                    std::process::exit(0)
                },
                Player::Computer => {
                    println!("{} \n I won human, you will never defeat me!", self);
                    std::process::exit(0)
                },
                Player::None => panic!(),
            }
        }

        // Set next player
        self.player = match self.player {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
            Player::None => panic!(),
        };
    }

    // Board looks like this internally:
    // 0 1 2 
    // 3 4 5
    // 6 7 8
    fn is_terminal(&self) -> bool {

        // check columns
        for i in (0..=6).step_by(3) {
            if (self.board[i] == Player::Human) || (self.board[i] == Player::Computer) {
                if self.board[i] == self.board[i+1] && self.board[i+1] == self.board[i+2] {
                    return true
                }
            }
        }

        // check rows
        for i in 0..=2 {
            if (self.board[i] == Player::Human) || (self.board[i] == Player::Computer) {
                if self.board[i] == self.board[i+3] && self.board[i+3] == self.board[i+6] {
                    return true
                }
            }
        }

        // check diagonals
        // top left -> bottom right
        if (self.board[0] == Player::Human) || (self.board[0] == Player::Computer) {
            if self.board[0] == self.board[4] && self.board[4] == self.board[8] {
                return true
            }
        }
        // top right -> bottom left
        if (self.board[2] == Player::Human) || (self.board[2] == Player::Computer) {
            if self.board[2] == self.board[4] && self.board[4] == self.board[6] {
                return true
            }
        }

        false

    }

}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (count, v) in self.board.iter().enumerate() {
            if [0,3,6].contains(&count) {
                write!(f, "\n")?
            }
            match v {
                Player::None => write!(f,". ")?,
                Player::Human => write!(f,"x ")?,
                Player::Computer => write!(f,"o ")?,
            }
        }
        write!(f, "\n")
    }
}

impl MCTS<GameState> for GameState {
    
    fn available_moves(&self) -> Vec<GameState> {
        let legal_moves = self.legal_moves();
        let mut game_states = Vec::new();
        for mv in legal_moves {
            let mut gs = self.clone();
            gs.exec_mcts_move(Some(mv));
            game_states.push(gs);
        }
        game_states
    }

    fn terminate(&self) -> f32 {
        let mut gs = self.clone();
        loop {
             match gs.exec_mcts_move(gs.get_rng_move()) {
                Some(x) => return x as f32,
                None => (),
             }
        }
    }
}