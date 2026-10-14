use std::fs::read_to_string;
use std::net::SocketAddr;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use archipelago_server_rs::messages::client::ClientPacket;
use archipelago_server_rs::messages::server::datapackage::DataPackagePacket;
use archipelago_server_rs::messages::server::ServerPacket;
use archipelago_server_rs::messages::server::roominfo::RoomInfoPacket;
use archipelago_server_rs::messages::server::invalid::InvalidPacketPacket;
use archipelago_server_rs::messages::server::invalid::PacketProblemType::Arguments;
use archipelago_server_rs::seed::Seed;
use archipelago_server_rs::send_packet;
use archipelago_server_rs::state::MultiworldState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/nathano/Archipelago/output/example_seed.json";
    let seed = Arc::new(serde_json::from_str::<Seed>(&*read_to_string(path).unwrap()).unwrap());
    // println!("{:?}", seed);

    let state = MultiworldState::new(&seed);

    let server_state = Arc::new(Mutex::new(state));


    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on: 127.0.0.1:3000");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        let state = server_state.clone();
        tokio::spawn(handle_connection(stream, addr, state, Arc::clone(&seed)));
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, state: Arc<Mutex<MultiworldState>>, seed: Arc<Seed>) {
    let Ok(ws_stream) = accept_async(stream).await else {
        return;
    };
    println!("Connection opened with adress {}", addr);
    let (mut tx, mut rx) = ws_stream.split();
    let mut connected = false;
    let state = state.lock().await;
    send_packet(&mut tx, ServerPacket::RoomInfo(RoomInfoPacket::new(&seed, &state))).await;
    while (!connected) {
        if let Some(Ok(msg)) = rx.next().await {
            match msg {
                Message::Text(raw_msg)  => {
                    let Ok(packet) = serde_json::from_slice::<ClientPacket>(raw_msg.as_ref()) else {
                        println!("Client sent malformed packet");
                        return;
                    };

                    match packet {
                        ClientPacket::GetDataPackage(gdppacket) => {
                            if let Ok(return_packet) = DataPackagePacket::new(&seed, gdppacket.games) {
                                send_packet(&mut tx, ServerPacket::DataPackage(return_packet)).await;
                            } else {
                                send_packet(&mut tx, ServerPacket::InvalidPacket(
                                    InvalidPacketPacket {
                                        problem_type: Arguments,
                                        original_cmd: Some("GetDataPackage".into()),
                                        text: "Invalid Games".into()
                                    })).await;
                            }
                        }
                    }
                },
                Message::Ping(_) | Message::Pong(_) => {},
                Message::Close(_) => {
                    println!("Connection closed at {}", addr);
                    let _ = tx.close().await;
                    return;
                },
                _ => {
                    eprintln!("Client sent unexpected ws message");
                }
            }

        } else {
            println!("Connection closed at {}", addr);
            return;
        }
    }

}