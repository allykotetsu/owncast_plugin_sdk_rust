use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::prelude::Event;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EventType {
    #[serde(rename = "chat.message.received")]
    ChatMessageReceived,
    #[serde(rename = "chat.user.joined")]
    ChatUserJoined,
    #[serde(rename = "chat.user.parted")]
    ChatUserParted,
    #[serde(rename = "chat.user.renamed")]
    ChatUserRenamed,
    #[serde(rename = "chat.message.moderated")]
    ChatMessageModerated,

    #[serde(rename = "stream.started")]
    StreamStarted,
    #[serde(rename = "stream.stopped")]
    StreamStopped,
    #[serde(rename = "stream.title.changed")]
    StreamTitleChanged,

    #[serde(rename = "sse.connect")]
    SseConnect,
    #[serde(rename = "sse.disconnect")]
    SseDisconnect,

    #[serde(rename = "tick")]
    Tick,

    #[serde(rename = "fediverse.activity")]
    FediverseActivity,
    #[serde(rename = "fediverse.follow")]
    FediverseFollow,
    #[serde(rename = "fediverse.like")]
    FediverseLike,
    #[serde(rename = "fediverse.repost")]
    FediverseRepost,
    #[serde(rename = "fediverse.quote")]
    FediverseQuote,
    #[serde(rename = "fediverse.mention")]
    FediverseMention,
    #[serde(rename = "fediverse.reply")]
    FediverseReply,

    #[serde(rename = "chat.command")]
    ChatCommand,
    #[serde(rename = "timer.fire")]
    TimerFire,

    Custom(String)
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl From<Event> for EventType {
    fn from(event: Event) -> Self {
        match event {
            Event::ChatMessageReceived { .. } => Self::ChatMessageReceived,
            Event::ChatUserJoined { .. } => Self::ChatUserJoined,
            Event::ChatUserParted { .. } => Self::ChatUserParted,
            Event::ChatUserRenamed { .. } => Self::ChatUserRenamed,
            Event::ChatMessageModerated { .. } => Self::ChatMessageModerated,

            Event::StreamStarted { .. } => Self::StreamStarted,
            Event::StreamStopped { .. } => Self::StreamStopped,
            Event::StreamTitleChanged { .. } => Self::StreamTitleChanged,

            Event::SseConnect { .. } => Self::SseConnect,
            Event::SseDisconnect { .. } => Self::SseDisconnect,

            Event::Tick { .. } => Self::Tick,

            Event::FediverseActivity { .. } => Self::FediverseActivity,
            Event::FediverseFollow { .. } => Self::FediverseFollow,
            Event::FediverseLike { .. } => Self::FediverseLike,
            Event::FediverseRepost { .. } => Self::FediverseRepost,
            Event::FediverseQuote { .. } => Self::FediverseQuote,
            Event::FediverseMention { .. } => Self::FediverseMention,
            Event::FediverseReply { .. } => Self::FediverseReply,

            Event::ChatCommand { .. } => Self::ChatCommand,
            Event::TimerFire { .. } => Self::TimerFire,

            Event::Custom { event_type, .. } => Self::Custom(event_type)
        }
    }
}