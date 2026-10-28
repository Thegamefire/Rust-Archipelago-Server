use serde::{Deserialize, Serialize};
use crate::structs::LocationId;

#[derive(Deserialize, Serialize)]
pub struct LocationChecksPacket {
    pub locations: Vec<LocationId>
}