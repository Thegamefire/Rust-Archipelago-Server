use std::collections::{HashMap, HashSet};
use crate::structs::{LocationId, SlotId};

pub struct MultiworldState {
    collected_locations: HashMap<SlotId, HashSet<LocationId>>,
    hinted_locations: HashMap<SlotId, HashSet<LocationId>>
}
impl MultiworldState {
    pub fn new(slots: Vec<SlotId>) -> MultiworldState {
        let mut s = MultiworldState {
            collected_locations: HashMap::new(),
            hinted_locations: HashMap::new()
        };
        for slot in slots {
            s.collected_locations.insert(slot, HashSet::new());
            s.hinted_locations.insert(slot, HashSet::new());
        }
        return s;
    }
}