use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use derive_more::Display;

#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub struct ItemId(i64);
#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LocationId(i64);
#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub struct SlotId(u32);
#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Clone)]
pub struct SlotName(String);
#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Clone)]
pub struct SlotAlias(String);
#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Clone)]
pub struct GameName(String);
impl GameName {
    pub fn empty() -> Self {
        GameName("".into())
    }
}
#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub struct TeamId(u32);
impl TeamId {
    pub fn not_implemented() -> Self {
        // Teams aren't implemented in the official MultiServer that I can find,
        // So it's not a priority right now.
        TeamId(0)
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum SlotType {
    Spectator = 0, Player = 1, Group = 2
}