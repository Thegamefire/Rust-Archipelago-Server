use std::collections::{HashMap, HashSet};
use crate::seed::Seed;
use crate::structs::{LocationId, SlotId};

pub struct MultiworldState {
    collected_locations: HashMap<SlotId, HashSet<LocationId>>,
    hinted_locations: HashMap<SlotId, HashSet<LocationId>>
}
impl MultiworldState {
    pub fn new(seed: &Seed) -> MultiworldState {
        let mut s = MultiworldState {
            collected_locations: HashMap::new(),
            hinted_locations: HashMap::new(),
        };
        for slot in seed.slot_ids() {
            s.collected_locations.insert(slot, HashSet::new());
            s.hinted_locations.insert(slot, HashSet::new());
        }
        return s;
    }
}