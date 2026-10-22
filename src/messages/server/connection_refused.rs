use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectionRefusedPacket {
    pub errors: Vec<ConnectionRefusedReason>
}


#[derive(Serialize, Deserialize)]
pub enum ConnectionRefusedReason {
    InvalidSlot, InvalidGame, IncompatibleVersion, InvalidPassword, InvalidItemsHandling
}