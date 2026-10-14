use serde::{Deserialize, Serialize};
use crate::structs::GameName;

#[derive(Serialize, Deserialize)]
pub struct GetDataPackagePacket {
    pub games: Option<Vec<GameName>>
}