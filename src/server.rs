use std::{
    sync::{Arc, Mutex},
    thread,
    collections::HashMap,
    error::Error,
};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_channel::mpsc::{unbounded, UnboundedSender};

fn server_abs() -> Result<(), Box<dyn Error>> {
    type Tx = UnboundedSender<Message>;
    let shared_state: Arc<Mutex<HashMap<String, Tx>>> = Arc::new(Mutex::new(HashMap::new()));

   Ok(()) 
}
