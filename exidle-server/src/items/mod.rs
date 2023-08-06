use serde::{Serialize, Deserialize};
use rand::{prelude::*, distributions::WeightedIndex};
use rand_chacha::ChaCha8Rng;

#[derive(Serialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub damage: u64,
    pub attack_spd: f32,
    pub item_type: String,
    pub rarity: Rarity
}

#[derive(Deserialize)]
pub struct ItemGenerator {
    pub seed: u64
}

#[derive(Serialize)]
pub enum Rarity {
    NORMAL,
    MAGIC,
    RARE
}

impl Item {
    pub fn generate(&self, seed:u64) {
        //Generate a random item based on seed value
        //let rng = rand::SeedableRng::from_seed(seed);
        let rarities = [(Rarity::NORMAL, 3),(Rarity::MAGIC, 2),(Rarity::RARE, 1)];
        let weight_dist =  WeightedIndex::new(rarities.iter().map(|(_,rarity)| rarity)).unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        println!("A num is: {}", rng.gen::<u64>()); 
        println!("Rarity is: {}", weight_dist.sample(&rng));
    }
}

#[cfg(test)]
mod tests {
    use super::{Item, Rarity};

    #[test]
    fn try_gen() {
        let seed = 64;
        let item = Item {
            id:1,
            name: String::from("Sword"),
            damage: 5,
            attack_spd: 1.0,
            item_type: String::from("one-hannded"),
            rarity: Rarity::NORMAL
        };
       item.generate(seed); 
    }
}