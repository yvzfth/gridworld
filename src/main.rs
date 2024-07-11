// main.rs
mod state;
mod action;
mod q_table;
mod grid_world;

use crate::grid_world::GridWorld;
use std::env;

const MODEL_PATH: &str = "q_table.bin";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut grid_world = GridWorld::new();

    if args.len() > 1 && args[1] == "train" {
        grid_world.train_model(10000); // Train with 10000 episodes
        grid_world.save_q_table(MODEL_PATH).expect("Failed to save Q-table");
        println!("Training completed and Q-table saved.");
    } else {
        grid_world = GridWorld {
            q_table: GridWorld::load_q_table(MODEL_PATH).expect("Failed to load Q-table"),
        };
        grid_world.run_model();
    }
}