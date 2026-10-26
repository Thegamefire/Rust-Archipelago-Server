use crate::messages::NetworkItem;
use crate::seed::Seed;
use crate::structs::{LocationId, SlotAlias, SlotId, TeamId};
use std::collections::{HashMap, HashSet};

pub struct MultiworldState {
    collected_locations: HashMap<SlotId, HashSet<LocationId>>,
    hinted_locations: HashMap<SlotId, HashSet<LocationId>>,
    slot_aliases: HashMap<(TeamId, SlotId), SlotAlias>,
}
impl MultiworldState {
    pub fn new(seed: &Seed) -> MultiworldState {
        let mut s = MultiworldState {
            collected_locations: HashMap::new(),
            hinted_locations: HashMap::new(),
            slot_aliases: HashMap::new(),
        };
        for slot in seed.slot_ids() {
            s.collected_locations.insert(slot, HashSet::new());
            s.hinted_locations.insert(slot, HashSet::new());
        }
        s
    }
    pub fn missing_locations<'a>(&'a self, seed: &'a Seed, slot: &SlotId) -> Vec<&'a LocationId> {
        let all_locations = seed
            .locations
            .get(slot)
            .map(|v| v.keys().collect())
            .unwrap_or(vec![]);
        let collected_locations = self.collected_locations(slot);
        all_locations
            .into_iter()
            .filter(|loc| !collected_locations.contains(loc))
            .collect()
    }
    pub fn collected_locations(&self, slot: &SlotId) -> Vec<&LocationId> {
        self.collected_locations
            .get(slot)
            .map(|s| s.iter().collect())
            .unwrap_or(vec![])
    }

    pub fn received_items(&self, seed: &Seed, slot: &SlotId) -> Vec<NetworkItem> {
        let mut result = vec![];
        for (location_slot, location_mapping) in &seed.locations {
            for (location, item) in location_mapping {
                if item.1 == *slot {
                    result.push(NetworkItem::new(item.0, *location, *location_slot, item.2))
                }
            }
        }
        result
    }
}
