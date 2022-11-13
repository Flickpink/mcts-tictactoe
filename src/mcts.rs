use ordered_float::{OrderedFloat};

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub id: usize,
    pub parent: Option<usize>,
    pub game_state: T,
    pub visits: u32,
    pub score: f32,
    pub children: Vec<usize>,
    // pub UCB1: f32,
}

pub trait MCTS<T> {
    fn available_moves(&self) -> Vec<T>;
    fn terminate(&self) -> f32;
    // terminate: The game should be played from it's current state with random
    // moves until it reaches a terminal state. The returned value should be
    // the value of this result. In the simplest case for example:
    // -1 for a loss, 0 for a draw, +1 for a win.
}

#[derive(Debug)]
pub struct Arena<Node> {
    nodes: Vec<Node>,
}

impl<T> Node<T> {

    fn new_child(&self, arena: &Arena<Node<T>>) -> Node<T>
    where T: Clone + Default + MCTS<T>
    {
        Self {
            id: arena.next_id(),
            parent: Some(self.id),
            game_state: self.game_state.clone(),
            visits: 0,
            score: 0.0,
            children: Vec::new(),
            // UCB1: 0.0
        }
    }

    fn new_child_with_gamestate(&self, arena: &Arena<Node<T>>, gs: T) -> Node<T>
    where T: Clone + Default + MCTS<T>
    {
        Self {
            id: arena.next_id(),
            parent: Some(self.id),
            game_state: gs,
            visits: 0,
            score: 0.0,
            children: Vec::new(),
            // UCB1: 0.0,
        }
    }

    pub fn default() -> Node<T>
    where T: Default
    {
        Self {
            id: 0,
            parent: None,
            game_state: T::default(),
            visits: 0,
            score: 0.0,
            children: Vec::new(),
            // UCB1: 0.0,
        }
    }

    pub fn with_gs(gs: T) -> Node<T> {
        Self {
            id: 0,
            parent: None,
            game_state: gs,
            visits: 0,
            score: 0.0,
            children: Vec::new()
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn is_root(&self) -> bool {
        match self.parent {
            None => true,
            Some(_) => false,
        }
    }

}


impl<T> Arena<Node<T>>
    where T: Clone + Default + MCTS<T>
    {
    pub fn new() -> Self {
        Arena {
            nodes: vec![Node::default()],
        }
    }

    pub fn new_with_gamestate(gs: T) -> Self {
        Arena {
            nodes: vec![Node::with_gs(gs)]
        }
    }

    fn next_id(&self) -> usize {
        self.nodes.len()
    }

    pub fn child_from(&mut self, parent: usize){
        self.nodes.push(self[parent].new_child(&self))
        //self.nodes.last().unwrap().id
    }

    fn child_from_with_gamestate(&mut self, parent: usize, gs: T) {
        self.nodes.push(self[parent].new_child_with_gamestate(&self, gs))
    }

    // UCB1 is calculated in the impl of Arena because it required information
    // about the parent
    pub fn ucb1_of(&self, node: usize) -> OrderedFloat<f32> {
        OrderedFloat(self.exploitation_term(node) + self.exploration_term(node))
    }

    fn exploitation_term(&self, node: usize) -> f32 {
    // aka: average_reward
        self[node].score / self[node].visits as f32
    }

    fn exploration_term(&self, node: usize) -> f32 {

        let nominator = match self[node].parent {
            Some(x) => (self[x].visits as f32).ln(),
            None => NEG_INFINITY, // This should only be the output if 
        };

        2.0 * (nominator / self[node].visits as f32).sqrt()
    }

    fn rollout(&mut self, node: usize) {
        self[node].score = self[node].game_state.terminate();
        self[node].visits += 1;
        self.propagate_from(node);
    }

    pub fn propagate_from(&mut self, node: usize) {
        assert!(&self[node].is_leaf());
         
        let scr = self[node].score;
        let mut node = node;

        loop {
            match self[node].parent {
                Some(parent) => {
                    self[parent].score += scr;
                    self[parent].visits += 1;
                    node = self[parent].id;
                }
                None => break
            }
        }
    }

    pub fn add_children(&mut self, node: usize) {
        
        let available_moves = self[node].game_state.available_moves().into_iter();

        for gs in available_moves {
            self.child_from_with_gamestate(node, gs);
        }

    }

    pub fn iterate(&mut self, n: usize) {
        
        let mut current = 0;
        
        for _ in 0..n {
            match self[current].is_leaf() {
                true => {
                    match self[current].visits {
                        0 => self.rollout(current),
                        1 => self.add_children(current),
                        _ => panic!(),
                    }
                },
                false => {
                    current = *self[current].children.iter()
                    .max_by_key(|child|self.ucb1_of(**child)).unwrap();
                }
            }
        }
    }
}

use std::{ops::{Index,IndexMut, Deref}, f32::NEG_INFINITY};

use crate::tictactoe::{GameState};

impl<T> Index<usize> for Arena<Node<T>> {
    type Output = Node<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T> IndexMut<usize> for Arena<Node<T>> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

// impl<T> Iterator for Node<T> {
//     type Item = usize;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         self.parent
//     }
// }



// impl<T> Iterator for Iter<T> {
//     type Item = usize;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.parent {
//             Some(_) => self.parent,
//             None => None,
//         }
//     }
// }

