use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde_json::Error as JsonError;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::event::Event::Custom;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::sse_connection_event::SSEConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;

pub(crate) enum Event {
    // Chat events
    ChatMessageReceived(ChatMessage),
    ChatUserJoined(User),
    ChatUserParted(User),
    ChatUserRenamed(ChatUserRename),
    ChatMessageModerated(ChatMessageModeration),

    // Stream lifecycle
    StreamStarted(StreamStarted),
    StreamStopped(StreamStopped),
    StreamTitleChanged(StreamTitleChange),

    // SSE connection lifecycle (who connected to / left a plugin's stream)
    SSEConnect(SSEConnectionEvent),
    SSEDisconnect(SSEConnectionEvent),

    // Once-a-second tick for periodic work (opt in by defining onTick)
    Tick(TickEvent),

    // Fediverse, engagement (metadata only) + inbound posts (with content)
    FediverseActivity(HashMap<String, String>),
    FediverseFollow(FediverseEngagement),
    FediverseLike(FediverseTargetedEngagement),
    FediverseRepost(FediverseTargetedEngagement),
    FediverseQuote(FediverseTargetedEngagement),
    FediverseMention(FediverseInboundPost),
    FediverseReply(FediverseInboundPost),

    Custom(String, String)
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum Field { EventType, Payload }

        struct EventVisitor;
        impl<'de2> Visitor<'de2> for EventVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                formatter.write_str("enum Event")
            }

            fn visit_seq<V: SeqAccess<'de2>>(self, mut seq: V) -> Result<Event, V::Error> {
                let event_type = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let payload = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;
                TryFrom::<(&str, &str)>::try_from((event_type, payload)).map_err(Error::custom)
            }

            fn visit_map<V: MapAccess<'de2>>(self, mut map: V) -> Result<Event, V::Error> {
                let mut event_type = None;
                let mut payload = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::EventType => {
                            if event_type.is_some() {
                                return Err(Error::duplicate_field("event_type"));
                            }
                            event_type = Some(map.next_value()?);
                        }
                        Field::Payload => {
                            if payload.is_some() {
                                return Err(Error::duplicate_field("payload"));
                            }
                            payload = Some(map.next_value()?);
                        }
                    }
                }

                let event_type = event_type.ok_or_else(|| Error::missing_field("event_type"))?;
                let payload = payload.ok_or_else(|| Error::missing_field("payload"))?;
                TryFrom::<(&str, &str)>::try_from((event_type, payload)).map_err(Error::custom)
            }
        }

        deserializer.deserialize_struct("", &["event_type", "payload"], EventVisitor)
    }
}

impl TryFrom<(&str, &str)> for Event {
    type Error = JsonError;

    fn try_from((event_type, payload): (&str, &str)) -> Result<Self, Self::Error> {
        match event_type {
            "chat.message.received" => Ok(Event::ChatMessageReceived(serde_json::from_str(payload)?)),
            "chat.user.joined" => Ok(Event::ChatUserJoined(serde_json::from_str(payload)?)),
            "chat.user.parted" => Ok(Event::ChatUserParted(serde_json::from_str(payload)?)),
            "chat.user.renamed" => Ok(Event::ChatUserRenamed(serde_json::from_str(payload)?)),
            "chat.message.moderated" => Ok(Event::ChatMessageModerated(serde_json::from_str(payload)?)),

            "stream.started" => Ok(Event::StreamStarted(serde_json::from_str(payload)?)),
            "stream.stopped" => Ok(Event::StreamStopped(serde_json::from_str(payload)?)),
            "stream.title.changed" => Ok(Event::StreamTitleChanged(serde_json::from_str(payload)?)),

            "sse.connect" => Ok(Event::SSEConnect(serde_json::from_str(payload)?)),
            "sse.disconnect" => Ok(Event::SSEDisconnect(serde_json::from_str(payload)?)),

            "tick" => Ok(Event::Tick(serde_json::from_str(payload)?)),

            "fediverse.activity" => Ok(Event::FediverseActivity(serde_json::from_str(payload)?)),
            "fediverse.follow" => Ok(Event::FediverseFollow(serde_json::from_str(payload)?)),
            "fediverse.like" => Ok(Event::FediverseLike(serde_json::from_str(payload)?)),
            "fediverse.repost" => Ok(Event::FediverseRepost(serde_json::from_str(payload)?)),
            "fediverse.quote" => Ok(Event::FediverseQuote(serde_json::from_str(payload)?)),
            "fediverse.mention" => Ok(Event::FediverseMention(serde_json::from_str(payload)?)),
            "fediverse.reply" => Ok(Event::FediverseReply(serde_json::from_str(payload)?)),

            _ => Ok(Custom(event_type.to_string(), payload.to_string()))
        }
    }
}