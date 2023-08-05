use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub damage: u64,
    pub attack_spd: f32,
    pub item_type: String
}

#[derive(Deserialize)]
pub struct ItemGenerator {
    pub seed: u64
}