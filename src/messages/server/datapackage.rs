use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::seed::{GameDatapackagePart, Seed};
use crate::structs::GameName;

#[derive(Serialize, Deserialize)]
pub struct DataPackagePacket {
    data: DatapackageObject
}

#[derive(Serialize, Deserialize)]
pub struct DatapackageObject {
    games: HashMap<GameName, GameDatapackagePart>
}
impl DataPackagePacket {
    pub fn new(seed: &Seed, games: Option<Vec<GameName>>) -> Result<DataPackagePacket, ()> {
        let mut result: HashMap<GameName, GameDatapackagePart> = HashMap::new();
        for game in games.unwrap_or(seed.games()) {
            let Some(dp_part) = seed.datapackage.get(&game) else {
                println!("PACKETERROR Invalid Game in GetDataPackagePacket");
                return Err(());
            };
            result.insert(game, dp_part.clone());
        }
        Ok(DataPackagePacket {
            data: DatapackageObject { games: result },
        })
    }
}