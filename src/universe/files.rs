use super::types;
use std::fs;

pub fn get_state_from_file(state_file: &str) -> types::State {
    let content = fs::read_to_string(state_file).expect("Error while reading file");
    let state: types::State = serde_json::from_str(&content).unwrap();
    state
}
