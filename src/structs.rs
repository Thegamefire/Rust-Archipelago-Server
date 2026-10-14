use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct ItemId(i64);
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LocationId(i64);
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct SlotId(u32);
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SlotName(String);
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct GameName(String);

