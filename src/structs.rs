use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct ItemId(i64);
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct LocationId(i64);
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct SlotId(u32);