// state.rs
use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct State {
    pub x: usize,
    pub y: usize,
}
