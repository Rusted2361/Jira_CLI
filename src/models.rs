use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    Inprogress,
    Resolved,
    Closed,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Story {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
