// q_table.rs
use std::collections::HashMap;
use crate::state::State;
use crate::action::Action;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QTable {
    pub table: HashMap<(State, Action), f64>,
}

impl QTable {
    pub fn new() -> Self {
        QTable {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, state: &State, action: &Action) -> Option<&f64> {
        self.table.get(&(*state, *action))
    }

    pub fn insert(&mut self, state: State, action: Action, value: f64) {
        self.table.insert((state, action), value);
    }
}
