use crate::connection::CONNECTIONS;
use crate::messages::server::ServerPacket;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

pub mod connection;
pub mod messages;
pub mod seed;
pub mod state;
pub mod structs;

pub async fn send_packet(id: Uuid, packet: ServerPacket) {
    let packets = vec![packet];
    send_packets(id, packets).await;
}
pub async fn send_packets(id: Uuid, packet: Vec<ServerPacket>) {
    let connections = CONNECTIONS.read().await;
    let connection = connections.get(&id).unwrap();
    connection
        .tx
        .lock()
        .await
        .send(Message::Text(
            serde_json::to_string(&packet).unwrap().into(),
        ))
        .await
        .expect("Failed to send packet to client");
}
