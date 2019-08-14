mod cell;

pub use cell::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Cells {
    cell: Vec<Cell>,
}

impl Cells {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cell.iter()
    }
}
