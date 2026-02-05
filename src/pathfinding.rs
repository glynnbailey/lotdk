use crate::actor_manager::ActorManager;
use crate::map_manager::MapManager;
use crate::position::Position;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
struct Node {
    position: Position,
    f_score: f32, // never directly read, used for ordering
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

pub fn a_star(actors: &ActorManager, map: &MapManager, start: Position, goal: Position, actor_id: usize) -> Option<Vec<Position>> {
    if start == goal {
        return Some(Vec::new());
    }

    let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
    let mut open_positions: HashSet<Position> = HashSet::new();
    let mut closed_set: HashSet<Position> = HashSet::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_scores: HashMap<Position, f32> = HashMap::new();
    let mut nodes_visited = 0;
    const MAX_NODES: usize = 250;

    g_scores.insert(start.clone(), 0.0);

    let start_node = Node {
        position: start.clone(),
        f_score: start.octile_distance(goal),
    };
    open_set.push(start_node);
    open_positions.insert(start.clone());

    while let Some(current) = open_set.pop() {
        if nodes_visited > MAX_NODES {
            return None;
        }
        nodes_visited += 1;

        open_positions.remove(&current.position);
        closed_set.insert(current.position.clone());

        // if the goal is found build and return the path
        if current.position == goal {
            let mut current_position = current.position;
            let mut path = Vec::new();

            // Build path excluding the start position
            while let Some(prev) = came_from.get(&current_position) {
                path.push(current_position.clone());
                current_position = prev.clone();
            }
            path.reverse();
            return Some(path);
        }

        for (neighbour, movement_cost_multiplier) in current.position.get_neighbours() {
            if closed_set.contains(&neighbour) {
                continue;
            }

            // check for obstacles
            if let Some(other_actor_id) = map.get_tile(neighbour).unwrap().actor_id() {
                if other_actor_id != actor_id && actors.get_actor(actor_id).unwrap().is_friendly_towards(&actors.get_actor(other_actor_id).unwrap()) {
                    continue;
                }
            }

            // calculate the g score, which is the score to get from the start to this tile
            let tentative_g_score = g_scores.get(&current.position).unwrap() + (map.get_tile(neighbour).unwrap().movement_cost() as f32 * movement_cost_multiplier);

            // if the g score is better or doesnt exist (we have never checked this path before or this path is better than the previous ones) add it
            if tentative_g_score < *g_scores.get(&neighbour).unwrap_or(&f32::MAX) {
                came_from.insert(neighbour.clone(), current.position.clone());
                g_scores.insert(neighbour.clone(), tentative_g_score);

                // add it to the open set if not already there
                if !open_positions.contains(&neighbour) {
                    // Add tie-breaking bias toward direct movement
                    let dx1 = neighbour.x - goal.x;
                    let dy1 = neighbour.y - goal.y;
                    let dx2 = start.x - goal.x;
                    let dy2 = start.y - goal.y;
                    let cross = (dx1 * dy2 - dx2 * dy1).abs() as f32;
                    let tie_breaker = cross * 0.001; // Small bias

                    let f_score = tentative_g_score + neighbour.octile_distance(goal) + tie_breaker;
                    open_set.push(Node { position: neighbour.clone(), f_score });
                    open_positions.insert(neighbour);
                }
            }
        }
    }

    None
}
