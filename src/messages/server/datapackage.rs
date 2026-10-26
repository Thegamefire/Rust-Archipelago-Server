use crate::connection::ClientError;
use crate::seed::{GameDatapackagePart, Seed};
use crate::structs::GameName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct DataPackagePacket {
    data: DatapackageObject,
}

#[derive(Serialize, Deserialize)]
pub struct DatapackageObject {
    games: HashMap<GameName, GameDatapackagePart>,
}
impl DataPackagePacket {
    pub fn new(
        seed: &Seed,
        games: Option<Vec<GameName>>,
    ) -> Result<DataPackagePacket, ClientError> {
        let mut result: HashMap<GameName, GameDatapackagePart> = HashMap::new();
        for game in games.unwrap_or(seed.games()) {
            let Some(dp_part) = seed.datapackage.get(&game) else {
                return Err(ClientError::InvalidArguments {
                    command: String::from("GetDataPackage"),
                    text: String::from("Invalid Game"),
                });
            };
            result.insert(game, dp_part.clone());
        }
        Ok(DataPackagePacket {
            data: DatapackageObject { games: result },
        })
    }
}
