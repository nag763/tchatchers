use crate::ws_message::WsMessage;
use redis::Connection;

pub struct Room {
    pub messages: Vec<WsMessage>,
}

impl Room {
    pub fn find_all(conn: &mut Connection) -> Vec<String> {
        redis::cmd("KEYS").arg("*").query(conn).unwrap()
    }

    pub fn find_messages_in_room(conn: &mut Connection, room_name: &str) -> Vec<WsMessage> {
        let messages: Vec<String> = redis::cmd("LRANGE")
            .arg(room_name)
            .arg("0")
            .arg("-1")
            .query(conn)
            .unwrap();
        messages
            .iter()
            .map(|m| serde_json::from_str(m).unwrap())
            .collect()
    }

    pub fn publish_message_in_room(conn: &mut Connection, room_name: &str, ws_message: WsMessage) {
        redis::cmd("LPUSH")
            .arg(room_name)
            .arg(serde_json::to_string(&ws_message).unwrap())
            .query(conn)
            .unwrap()
    }
}
