use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadList {
    name: String,
    description: String,
}

impl DownloadList {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
