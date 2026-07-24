use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::prelude::Envelope;

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

impl From<Envelope> for EventType {
    fn from(event: Envelope) -> Self {
        match event {
            Envelope::ChatMessageReceived { .. } => Self::ChatMessageReceived,
            Envelope::ChatUserJoined { .. } => Self::ChatUserJoined,
            Envelope::ChatUserParted { .. } => Self::ChatUserParted,
            Envelope::ChatUserRenamed { .. } => Self::ChatUserRenamed,
            Envelope::ChatMessageModerated { .. } => Self::ChatMessageModerated,

            Envelope::StreamStarted { .. } => Self::StreamStarted,
            Envelope::StreamStopped { .. } => Self::StreamStopped,
            Envelope::StreamTitleChanged { .. } => Self::StreamTitleChanged,

            Envelope::SseConnect { .. } => Self::SseConnect,
            Envelope::SseDisconnect { .. } => Self::SseDisconnect,

            Envelope::Tick { .. } => Self::Tick,

            Envelope::FediverseActivity { .. } => Self::FediverseActivity,
            Envelope::FediverseFollow { .. } => Self::FediverseFollow,
            Envelope::FediverseLike { .. } => Self::FediverseLike,
            Envelope::FediverseRepost { .. } => Self::FediverseRepost,
            Envelope::FediverseQuote { .. } => Self::FediverseQuote,
            Envelope::FediverseMention { .. } => Self::FediverseMention,
            Envelope::FediverseReply { .. } => Self::FediverseReply,

            Envelope::ChatCommand { .. } => Self::ChatCommand,
            Envelope::TimerFire { .. } => Self::TimerFire,

            Envelope::Custom { event_type, .. } => Self::Custom(event_type)
        }
    }
}