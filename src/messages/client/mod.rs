pub mod connect;
pub mod connect_update;
mod get_datapackage;

use crate::messages::client::connect::ConnectPacket;
use crate::messages::client::connect_update::ConnectUpdatePacket;
use crate::messages::client::get_datapackage::GetDataPackagePacket;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientPacket {
    GetDataPackage(GetDataPackagePacket),
    Connect(ConnectPacket),
    ConnectUpdate(ConnectUpdatePacket),
    Sync,
}
