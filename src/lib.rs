use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::messages::server::ServerPacket;

pub mod seed;
pub mod structs;
pub mod state;
pub mod messages;
pub mod connection;

pub async fn send_packet(tx: &mut SplitSink<WebSocketStream<TcpStream>, Message>, packet: ServerPacket) {
    tx.send(Message::Text(serde_json::to_string(&vec![packet]).unwrap().into())).await.expect("Failed to send packet to client");
}
pub async fn send_packets(tx: &mut SplitSink<WebSocketStream<TcpStream>, Message>, packet: Vec<ServerPacket>) {
    tx.send(Message::Text(serde_json::to_string(&packet).unwrap().into())).await.expect("Failed to send packet to client");
}