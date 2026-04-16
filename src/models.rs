use chrono::Utc;
use sha2::{Sha256, Digest};

struct MsgFormat {
    username: String,
    uuid: String,
}

impl MsgFormat {
    fn new(username: String) -> Self {

        let uuid = Self::get_uuid(&username);

        MsgFormat {
            username: username, 
            uuid: uuid,
        }    
    }

    fn get_uuid(username: &String) -> String{
        let time_stamp = Utc::now().timestamp_nanos_opt().unwrap();      

        let concatenated_string = username.clone() + &time_stamp.to_string();

        let hash_string = Sha256::digest(&concatenated_string);
        let hash_byte = hash_string.iter().map(|b| format!("{:02X}", b)).collect::<String>();
        hash_byte
    }
}
