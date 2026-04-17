use tokio::sync::mpsc;
use crate::models;

pub async fn db_task(mut msg: mpsc::Receiver<String>) -> mpsc::Sender<String> {
   tokio::spawn(async move { 
       while let Some(rx) = msg.recv.await {
           models::MsgFormat::new(rx)

          } 
       }
   )
}
