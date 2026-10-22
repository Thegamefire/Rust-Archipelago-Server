pub mod datapackage;
pub mod roominfo;
pub mod invalid;
pub mod connected;
pub mod connection_refused;

use serde::{Deserialize, Serialize};
use crate::messages::server::datapackage::DataPackagePacket;
use crate::messages::server::invalid::InvalidPacketPacket;
use crate::messages::server::roominfo::RoomInfoPacket;
use crate::messages::server::connected::ConnectedPacket;
use crate::messages::server::connection_refused::ConnectionRefusedPacket;

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ServerPacket {
    RoomInfo(RoomInfoPacket),
    DataPackage(DataPackagePacket),
    InvalidPacket(InvalidPacketPacket),
    Connected(ConnectedPacket),
    ConnectionRefused(ConnectionRefusedPacket)
}
