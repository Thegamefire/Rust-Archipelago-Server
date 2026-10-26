use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InvalidPacketPacket {
    #[serde(rename = "type")]
    pub problem_type: PacketProblemType,
    pub original_cmd: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub enum PacketProblemType {
    Cmd,
    Arguments,
}
