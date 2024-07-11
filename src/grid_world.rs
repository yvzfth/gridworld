
// grid_world.rs
use crate::state::State;
use crate::action::Action;
use crate::q_table::QTable;
use rand::Rng;
use rand::seq::SliceRandom;
use std::io::{Read, Write};

const GRID_SIZE: usize = 20;
const ACTIONS: [Action; 4] = [Action::Up, Action::Down, Action::Left, Action::Right];
const ALPHA: f64 = 0.1; // Learning rate
const GAMMA: f64 = 0.9; // Discount factor
const EPSILON: f64 = 0.1; // Exploration rate

pub struct GridWorld {
    pub q_table: QTable,
}

impl GridWorld {
    pub fn new() -> Self {
        let q_table = QTable::new();
        GridWorld { q_table }
    }

    fn get_possible_actions(&self, state: State) -> Vec<Action> {
        let mut actions = vec![];
        if state.x > 0 {
            actions.push(Action::Left);
        }
        if state.x < GRID_SIZE - 1 {
            actions.push(Action::Right);
        }
        if state.y > 0 {
            actions.push(Action::Up);
        }
        if state.y < GRID_SIZE - 1 {
            actions.push(Action::Down);
        }
        actions
    }

    fn get_next_state(&self, state: State, action: Action) -> State {
        match action {
            Action::Up if state.y > 0 => State { x: state.x, y: state.y - 1 },
            Action::Down if state.y < GRID_SIZE - 1 => State { x: state.x, y: state.y + 1 },
            Action::Left if state.x > 0 => State { x: state.x - 1, y: state.y },
            Action::Right if state.x < GRID_SIZE - 1 => State { x: state.x + 1, y: state.y },
            _ => state, // If the action would move the agent out of bounds, stay in the same state
        }
    }

    fn get_reward(&self, state: State) -> f64 {
        if state == (State { x: GRID_SIZE - 1, y: GRID_SIZE - 1 }) {
            1.0
        } else {
            -0.1
        }
    }

    pub fn train_model(&mut self, num_episodes: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_episodes {
            let mut state = State { x: 0, y: 0 };

            while state != (State { x: GRID_SIZE - 1, y: GRID_SIZE - 1 }) {
                let action = if rng.gen::<f64>() < EPSILON {
                    // Explore: choose a random action
                    *self.get_possible_actions(state).choose(&mut rng).unwrap()
                } else {
                    // Exploit: choose the best action from Q-table
                    let mut best_action = ACTIONS[0];
                    let mut best_value = f64::NEG_INFINITY;
                    for &action in &ACTIONS {
                        let value = *self.q_table.get(&state, &action).unwrap_or(&0.0);
                        if value > best_value {
                            best_value = value;
                            best_action = action;
                        }
                    }
                    best_action
                };

                let next_state = self.get_next_state(state, action);
                let reward = self.get_reward(next_state);

                let old_value = self.q_table.get(&state, &action).unwrap_or(&0.0);
                let next_max = ACTIONS
                    .iter()
                    .map(|&a| *self.q_table.get(&next_state, &a).unwrap_or(&0.0))
                    .fold(f64::NEG_INFINITY, f64::max);

                let new_value = old_value + ALPHA * (reward + GAMMA * next_max - old_value);
                self.q_table.insert(state, action, new_value);

                state = next_state;
            }
        }
    }

    pub fn save_q_table(&self, path: &str) -> std::io::Result<()> {
        let serialized = bincode::serialize(&self.q_table).unwrap();
        let mut file = std::fs::File::create(path)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    pub fn load_q_table(path: &str) -> std::io::Result<QTable> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let q_table: QTable = bincode::deserialize(&buffer).unwrap();
        Ok(q_table)
    }

    pub fn print_grid(&self, state: State) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if state.x == x && state.y == y {
                    print!("A "); // Agent's position
                } else if x == GRID_SIZE - 1 && y == GRID_SIZE - 1 {
                    print!("G "); // Goal position
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!();
    }

    pub fn run_model(&self) {
        let mut state = State { x: 0, y: 0 };

        println!("Loaded Q-table:");

        println!("Running the model:");
        while state != (State { x: GRID_SIZE - 1, y: GRID_SIZE - 1 }) {
            self.print_grid(state);

            let mut best_action = ACTIONS[0];
            let mut best_value = f64::NEG_INFINITY;
            for &action in &ACTIONS {
                let value = *self.q_table.get(&state, &action).unwrap_or(&0.0);
                if value > best_value {
                    best_value = value;
                    best_action = action;
                }
            }

            state = self.get_next_state(state, best_action);
        }

        println!("Reached goal at: ({}, {})", state.x, state.y);
        self.print_grid(state); // Print the final state
    }
}
