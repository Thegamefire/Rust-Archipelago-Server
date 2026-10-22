mod get_datapackage;
pub mod connect;

use serde::{Deserialize, Serialize};
use crate::messages::client::get_datapackage::{GetDataPackagePacket};
use crate::messages::client::connect::{ConnectPacket};

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientPacket {
    GetDataPackage(GetDataPackagePacket),
    Connect(ConnectPacket)
}

