use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Player{
    pub id: u64,
    pub name: String
}
