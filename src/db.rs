use anyhow::Result;

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        // read the content's of self.file_path and deserialize it using serde
        // Read the contents of the file at self.file_path
        let file_content = std::fs::read_to_string(&self.file_path)?;
        // Deserialize the JSON content into a DBState object using Serde
        let db_state: DBState = serde_json::from_str(&file_content)?;

        Ok(db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        todo!() // serialize db_state to json and store it in self.file_path
    }
}