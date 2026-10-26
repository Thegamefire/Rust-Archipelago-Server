use archipelago_server_rs::connection::ConnectionManager;
use archipelago_server_rs::seed::Seed;
use archipelago_server_rs::state::MultiworldState;
use std::fs::read_to_string;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/nathano/Archipelago/output/example_seed.json";
    let seed = Arc::new(serde_json::from_str::<Seed>(&read_to_string(path).unwrap()).unwrap());
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

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    state: Arc<Mutex<MultiworldState>>,
    seed: Arc<Seed>,
) {
    let Ok(ws_stream) = accept_async(stream).await else {
        return;
    };
    println!("Connection opened with adress {}", addr);
    let conn = ConnectionManager::new(addr, ws_stream, state, seed);
    conn.start_listening().await;
}
