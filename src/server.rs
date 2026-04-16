use std::{
    sync::{Arc, Mutex},
    thread,
    collections::HashMap,
    error::Error,
    net::{SocketAddr, TcpStream},
};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_channel::mpsc::{unbounded, UnboundedSender};


type Tx = UnboundedSender<Message>;


async fn handle_connection() {
    let chat_room: HashMap<String, Tx> = HashMap::new();

    //println!("Incoming Tcp connection from: {}");



}
