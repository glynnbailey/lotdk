use crate::actor::Actor;
use std::collections::{BinaryHeap, HashMap};

pub struct ActorManager {
    actors: HashMap<usize, Actor>,
    turn_queue: BinaryHeap<ActorTurn>,
    current_turn: Option<ActorTurn>,
    next_id: usize,
}

impl ActorManager {
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
            turn_queue: BinaryHeap::new(),
            current_turn: None,
            next_id: 0,
        }
    }

    pub fn add_actor(&mut self, actor: Actor) -> usize {
        self.actors.insert(self.next_id, actor);
        self.turn_queue.push(ActorTurn::new(self.next_id, 0));
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn remove_actor(&mut self, actor_id: usize) {
        self.actors.remove(&actor_id);
        self.turn_queue.retain(|turn| turn.actor_id != actor_id);
        if let Some(current) = &self.current_turn {
            if current.actor_id == actor_id {
                self.current_turn = None;
            }
        }
    }

    pub fn get_actor(&self, actor_id: usize) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    pub fn get_actor_mut(&mut self, actor_id: usize) -> Option<&mut Actor> {
        self.actors.get_mut(&actor_id)
    }

    pub fn get_player_actor(&self) -> &Actor {
        &self.actors.get(&0).unwrap()
    }

    pub fn get_player_actor_mut(&mut self) -> &mut Actor {
        self.actors.get_mut(&0).unwrap()
    }

    pub fn take_player_actor(&mut self) -> Actor {
        self.actors.remove(&0).unwrap()
    }

    pub fn next_turn(&mut self) -> Option<usize> {
        match self.turn_queue.pop() {
            Some(turn) => {
                self.current_turn = Some(turn);
                let actor_id = self.current_turn.as_ref().unwrap().actor_id;
                return Some(actor_id);
            }
            None => unreachable!("Turn queue should never be empty"),
        }
    }

    pub fn end_turn(&mut self, cost: u32) {
        match self.current_turn.take() {
            Some(mut turn) => {
                turn.action_points += cost;
                self.turn_queue.push(turn);
            }
            None => unreachable!("No current turn to end"),
        }
    }
}

struct ActorTurn {
    actor_id: usize,
    action_points: u32,
}

impl ActorTurn {
    pub fn new(actor_id: usize, action_points: u32) -> Self {
        Self { actor_id, action_points }
    }
}

impl PartialEq for ActorTurn {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id == other.actor_id
    }
}

impl Eq for ActorTurn {}

impl PartialOrd for ActorTurn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ActorTurn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.action_points.cmp(&self.action_points).then_with(|| other.actor_id.cmp(&self.actor_id))
    }
}
