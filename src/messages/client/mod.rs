mod get_datapackage;

use serde::{Deserialize, Serialize};
use crate::messages::client::get_datapackage::{GetDataPackagePacket};

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientPacket {
    GetDataPackage(GetDataPackagePacket)
}

