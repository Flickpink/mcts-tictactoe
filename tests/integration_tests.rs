use tttlib::{mcts::{Arena, Node}, tictactoe::{GameState}};

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn create_arena() {
        let mut arena: Arena<Node<GameState>> = Arena::new();
        assert_eq!(arena[0],Node::default())
    }

    #[test]
    fn add_child_node() {
        let mut arena: Arena<Node<GameState>> = Arena::new();
        arena.child_from(0);
        let child = Node {
            id: 1,
            parent: Some(0),
            game_state: GameState::default(),
            visits: 0,
            score: 0.0,
            children: Vec::new(),
        };
        assert_eq!(arena[1],child)
    }
    
    #[test]
    fn traverse_branch() {
        let mut arena: Arena<Node<GameState>> = Arena::new();
        arena.child_from(0);
        arena.child_from(1);
        arena.child_from(2);
        arena[0].score = 3.0;
        arena[0].visits = 2;
        arena[1].score = 2.0;
        arena[1].visits = 1;
        arena[2].score = 3.0;
        arena.propagate_from(2);
        dbg!(&arena[0]);
        dbg!(&arena[1]);
        dbg!(&arena[2]);
        assert_eq!(arena[0].score, 6.0);
        assert_eq!(arena[1].score, 5.0);
        assert_eq!(arena[2].score, 3.0);
        assert_eq!(arena[0].visits, 3);
        assert_eq!(arena[1].visits, 2);
        assert_eq!(arena[2].visits, 0);

    }

}

