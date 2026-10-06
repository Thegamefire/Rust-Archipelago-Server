use std::cmp::min;
use serde::{Deserialize, Serialize, Serializer};

pub mod client;
pub mod server;

#[derive(Serialize, Deserialize)]
struct NetworkVersion {
    class: String,
    major: u16,
    minor: u16,
    build: u16,
}
impl NetworkVersion {
    fn new(major: u16, minor: u16, build: u16) {
        NetworkVersion {
            class: String::from("Version"),
            major,
            minor,
            build,
        };
    }
}

#[derive(Deserialize, Serialize)]
enum Permission {
    Disabled, Enabled, Goal, Auto, AutoEnabled
}