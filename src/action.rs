// action.rs
use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}
