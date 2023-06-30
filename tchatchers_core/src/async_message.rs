use derive_more::Display;
use redis::{
    from_redis_value,
    streams::{StreamReadOptions, StreamReadReply},
    Commands,
};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DEFAULT_EVENT_OPTIONS: StreamReadOptions =
        StreamReadOptions::default().block(0).count(100);
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
pub enum AsyncMessage {
    LoggedUsers(i32),
}

#[derive(Debug, Clone, Copy, Display)]
pub enum AsyncQueue {
    LoggedUsers,
}

impl AsyncMessage {
    fn get_queue(&self) -> AsyncQueue {
        match self {
            AsyncMessage::LoggedUsers(_) => AsyncQueue::LoggedUsers,
        }
    }

    pub fn spawn(&self, conn: &mut redis::Connection) {
        let self_as_str: String = serde_json::to_string(&self).unwrap();
        let queue_name = self.get_queue().to_string();
        let now = chrono::Utc::now();
        let event: [(&str, &String); 2] = [
            ("val", &self_as_str),
            ("timestamp", &now.timestamp().to_string()),
        ];
        redis::Cmd::xadd(&queue_name, "*", &event).execute(conn);
        debug!("[{queue_name}] Event registered");
    }

    pub fn read_events(
        queue: AsyncQueue,
        conn: &mut redis::Connection,
    ) -> Option<Vec<(String, Self)>> {
        let stream_events: Option<StreamReadReply> = conn
            .xread_options(&[&queue.to_string()], &["$"], &DEFAULT_EVENT_OPTIONS)
            .unwrap();
        if let Some(stream_events) = stream_events {
            let mut events: Vec<(String, Self)> = Vec::new();
            for keys in stream_events.keys {
                for id in keys.ids {
                    let event_id = id.id;
                    let map = id.map;
                    let Some(redis_serialized_value): Option<&redis::Value> = map.get("val") else {
                        error!("[{queue}] No value present for event #{event_id}");
                        continue;
                    };
                    let Ok(serialized_value) : Result<String, _> = from_redis_value(redis_serialized_value) else {
                        error!("Could not deserialize into string value for event #{event_id}");
                        continue;
                    };
                    let Ok(value) : Result<Self, _> = serde_json::from_str(&serialized_value) else {
                        error!("[{queue}] Could not deserialize string value for event #{event_id}");
                        continue;
                    };
                    debug!("[{queue}] Successfully parsed event #{event_id} and added it to queue response");
                    trace!("[{queue}] Parsed : \n{value:#?}");
                    events.push((event_id, value));
                }
            }
            debug!("[{queue}] Events fetched from queue : {}", events.len());
            trace!("[{queue}] Events : \n{:#?}", events);
            Some(events)
        } else {
            info!("[{queue}] No events found in queue, returning none");
            None
        }
    }
}
