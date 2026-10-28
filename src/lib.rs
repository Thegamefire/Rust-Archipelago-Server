use crate::connection::CONNECTIONS;
use crate::messages::server::ServerPacket;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;
use crate::messages::NetworkItem;
use crate::messages::server::received_items::ReceivedItemsPacket;
use crate::messages::server::ServerPacket::ReceivedItems;
use crate::structs::SlotId;

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

pub async fn send_to_slot(packet: ServerPacket, slot: SlotId) {
    let connections = CONNECTIONS.read().await;

    let result_packet = serde_json::to_string(&packet).unwrap();

    for connection in connections.values() {
        match &connection.connection_info {
            Some(info) if info.slot == slot => {
                connection
                    .tx
                    .lock()
                    .await
                    .send(Message::Text(
                        result_packet.clone().into()
                    ))
                    .await
                    .expect("Failed to send packet to client");
            },
            _ => {}
        }
    }
}

pub async fn send_items_to_slot(items: Vec<NetworkItem>, slot: SlotId) {
    let packets = {
        let mut connections = CONNECTIONS.write().await;
        let mut packets = Vec::new();

        for (c_id, connection) in connections.iter_mut() {
            if let Some(info) = &mut connection.connection_info {
                if info.slot == slot {
                    packets.push((
                        *c_id,
                        ServerPacket::ReceivedItems(ReceivedItemsPacket {
                            index: info.next_ri_index,
                            items: items.clone(),
                        }),
                    ));

                    info.next_ri_index += 1;
                }
            }
        }

        packets
    }; // The CONNECTIONS write lock is released here.

    for (c_id, packet) in packets {
        send_packet(c_id, packet).await;
    }
}