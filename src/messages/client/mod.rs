mod get_datapackage;
pub mod connect;
pub mod connect_update;

use serde::{Deserialize, Serialize};
use crate::messages::client::get_datapackage::{GetDataPackagePacket};
use crate::messages::client::connect::{ConnectPacket};
use crate::messages::client::connect_update::ConnectUpdatePacket;

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientPacket {
    GetDataPackage(GetDataPackagePacket),
    Connect(ConnectPacket),
    ConnectUpdate(ConnectUpdatePacket)
}

