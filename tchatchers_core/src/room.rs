//! Rooms are where user chats with each others.
//!
//! They are persisted within redis as "room_name=redis_key", so that any user
//! that reconnects retieve the messages sent before he joined.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::ws_message::WsMessage;
use redis::Connection;

/// A room gather a list of messages sent by users.
pub struct Room {
    /// The list of messages, ordered from the oldest to the newest.
    pub messages: Vec<WsMessage>,
}

impl Room {
    /// Returns all the messages persisted in a room.
    ///
    /// # Arguments
    ///
    /// - conn : The redis connection pool.
    /// - room_name : The name of the room.
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

    /// Adds a new message to a room.
    ///
    /// # Arguments
    ///
    /// - conn : The connection pool.
    /// - room_name : The name of the room.
    /// - ws_message : The message to be added to the room.
    pub fn publish_message_in_room(conn: &mut Connection, room_name: &str, ws_message: WsMessage) {
        redis::cmd("RPUSH")
            .arg(room_name)
            .arg(serde_json::to_string(&ws_message).unwrap())
            .query(conn)
            .unwrap()
    }
}
