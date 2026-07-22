use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde_json::Error as JsonError;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command_event::CommandEvent;
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

pub(crate) const CHAT_MESSAGE_RECEIVED: &str = "chat.message.received";
pub(crate) const CHAT_USER_JOINED: &str = "chat.user.joined";
pub(crate) const CHAT_USER_PARTED: &str = "chat.user.parted";
pub(crate) const CHAT_USER_RENAMED: &str = "chat.user.renamed";
pub(crate) const CHAT_MESSAGE_MODERATED: &str = "chat.message.moderated";

pub(crate) const STREAM_STARTED: &str = "stream.started";
pub(crate) const STREAM_STOPPED: &str = "stream.stopped";
pub(crate) const STREAM_TITLE_CHANGED: &str = "stream.title.changed";

pub(crate) const SSE_CONNECT: &str = "sse.connect";
pub(crate) const SSE_DISCONNECT: &str = "sse.disconnect";

pub(crate) const TICK: &str = "tick";

pub(crate) const FEDIVERSE_ACTIVITY: &str = "fediverse.activity";
pub(crate) const FEDIVERSE_FOLLOW: &str = "fediverse.follow";
pub(crate) const FEDIVERSE_LIKE: &str = "fediverse.like";
pub(crate) const FEDIVERSE_REPOST: &str = "fediverse.repost";
pub(crate) const FEDIVERSE_QUOTE: &str = "fediverse.quote";
pub(crate) const FEDIVERSE_MENTION: &str = "fediverse.mention";
pub(crate) const FEDIVERSE_REPLY: &str = "fediverse.reply";

pub(crate) const CHAT_COMMAND: &str = "chat.command";
pub(crate) const TIMER_FIRE: &str = "timer.fire";

pub enum Event {
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

    // Internal events
    ChatCommand(CommandEvent),
    TimerFire(),

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
            CHAT_MESSAGE_RECEIVED => Ok(Event::ChatMessageReceived(serde_json::from_str(payload)?)),
            CHAT_USER_JOINED => Ok(Event::ChatUserJoined(serde_json::from_str(payload)?)),
            CHAT_USER_PARTED => Ok(Event::ChatUserParted(serde_json::from_str(payload)?)),
            CHAT_USER_RENAMED => Ok(Event::ChatUserRenamed(serde_json::from_str(payload)?)),
            CHAT_MESSAGE_MODERATED => Ok(Event::ChatMessageModerated(serde_json::from_str(payload)?)),

            STREAM_STARTED => Ok(Event::StreamStarted(serde_json::from_str(payload)?)),
            STREAM_STOPPED => Ok(Event::StreamStopped(serde_json::from_str(payload)?)),
            STREAM_TITLE_CHANGED => Ok(Event::StreamTitleChanged(serde_json::from_str(payload)?)),

            SSE_CONNECT => Ok(Event::SSEConnect(serde_json::from_str(payload)?)),
            SSE_DISCONNECT => Ok(Event::SSEDisconnect(serde_json::from_str(payload)?)),

            TICK => Ok(Event::Tick(serde_json::from_str(payload)?)),

            FEDIVERSE_ACTIVITY => Ok(Event::FediverseActivity(serde_json::from_str(payload)?)),
            FEDIVERSE_FOLLOW => Ok(Event::FediverseFollow(serde_json::from_str(payload)?)),
            FEDIVERSE_LIKE => Ok(Event::FediverseLike(serde_json::from_str(payload)?)),
            FEDIVERSE_REPOST => Ok(Event::FediverseRepost(serde_json::from_str(payload)?)),
            FEDIVERSE_QUOTE => Ok(Event::FediverseQuote(serde_json::from_str(payload)?)),
            FEDIVERSE_MENTION => Ok(Event::FediverseMention(serde_json::from_str(payload)?)),
            FEDIVERSE_REPLY => Ok(Event::FediverseReply(serde_json::from_str(payload)?)),

            CHAT_COMMAND => Ok(Event::ChatCommand(serde_json::from_str(payload)?)),
            TIMER_FIRE => Ok(Event::TimerFire(/*serde_json::from_str(payload)?*/)),

            _ => Ok(Custom(event_type.to_string(), payload.to_string()))
        }
    }
}