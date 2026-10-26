use crate::structs::GameName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetDataPackagePacket {
    pub games: Option<Vec<GameName>>,
}
