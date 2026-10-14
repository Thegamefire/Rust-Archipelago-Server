pub mod datapackage;
pub mod roominfo;
pub mod invalid;

use serde::{Deserialize, Serialize};
use crate::messages::server::datapackage::DataPackagePacket;
use crate::messages::server::invalid::InvalidPacketPacket;
use crate::messages::server::roominfo::RoomInfoPacket;

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ServerPacket {
    RoomInfo(RoomInfoPacket),
    DataPackage(DataPackagePacket),
    InvalidPacket(InvalidPacketPacket)
}
