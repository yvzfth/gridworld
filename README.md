# Grid World Reinforcement Learning with Rust

This project implements a basic reinforcement learning (RL) agent in Rust to navigate a grid world environment. The agent learns to reach a goal position using Q-learning, a popular RL algorithm.

## Features

- **Grid Size:** The grid world is configurable with a size of 20x20.
- **Actions:** The agent can move Up, Down, Left, or Right.
- **Parameters:**
  - **Learning Rate (α):** 0.1
  - **Discount Factor (γ):** 0.9
  - **Exploration Rate (ε):** 0.1
  - **Number of Episodes:** 10000
- **Time Taken to Train:** Approximately 2.882 seconds.
- **Predicted Steps to Complete Grid World:** 2\*(n - 1) steps, where n is the grid size (20 in this case).

## Files

The project consists of the following main files:

- **state.rs:** Defines the State struct representing positions in the grid.
- **action.rs:** Defines the Action enum representing possible movements.
- **q_table.rs:** Manages the Q-table used for RL learning.
- **grid_world.rs:** Implements the GridWorld struct containing RL methods.
- **main.rs:** Entry point of the application handling training and running the model.

## Usage

1. **Training the Model:**

   ```sh
   cargo run train
   ```

   This command trains the RL agent for 10,000 episodes and saves the Q-table to `q_table.bin`.

2. **Running the Model:**
   ```sh
   cargo run
   ```
   This command loads the pre-trained Q-table from `q_table.bin` and runs the agent in the grid world.

## Dependencies

- `serde`: For serialization and deserialization of Q-table.
- `rand`: For generating random numbers and actions during training.
- `bincode`: For serializing the Q-table data to binary format.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Author

Fatih Yavuz
