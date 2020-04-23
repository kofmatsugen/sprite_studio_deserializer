use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CellMapNames {
    value: Vec<String>,
}

impl CellMapNames {
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.value.iter().map(|v| v.as_str())
    }
}
