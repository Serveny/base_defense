use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildingValues {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Building {
    Factory(BuildingValues),
    PowerPlant(BuildingValues),
}

#[allow(dead_code)]
impl Building {
    pub fn values_mut(&mut self) -> &mut BuildingValues {
        match self {
            Building::Factory(values) => values,
            Building::PowerPlant(values) => values,
        }
    }
}
