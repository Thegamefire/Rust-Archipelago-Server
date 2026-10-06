use std::fs::read_to_string;
use archipelago_server_rs::seed::RawSeed;
use archipelago_server_rs::state::MultiworldState;

fn main() {
    let path = "/home/nathano/Archipelago/output/AP_84615277806355214168.json";
    let seed = serde_json::from_str::<RawSeed>(&*read_to_string(path).unwrap()).unwrap();
    println!("{:?}", seed);

    let state = MultiworldState::new(seed.slot_ids());
}
