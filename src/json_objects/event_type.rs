use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::json_objects::envelope::Envelope;

/// A unique name representing an event. Used in [`Subscriptions`].
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EventType {
    /// The [`EventType`] for [`Envelope::ChatMessageReceived`]
    #[serde(rename = "chat.message.received")]
    ChatMessageReceived,

    /// The [`EventType`] for [`Envelope::ChatUserJoined`]
    #[serde(rename = "chat.user.joined")]
    ChatUserJoined,

    /// The [`EventType`] for [`Envelope::ChatUserParted`]
    #[serde(rename = "chat.user.parted")]
    ChatUserParted,

    /// The [`EventType`] for [`Envelope::ChatUserRenamed`]
    #[serde(rename = "chat.user.renamed")]
    ChatUserRenamed,

    /// The [`EventType`] for [`Envelope::ChatMessageModerated`]
    #[serde(rename = "chat.message.moderated")]
    ChatMessageModerated,

    /// The [`EventType`] for [`Envelope::StreamStarted`]
    #[serde(rename = "stream.started")]
    StreamStarted,

    /// The [`EventType`] for [`Envelope::StreamStopped`]
    #[serde(rename = "stream.stopped")]
    StreamStopped,

    /// The [`EventType`] for [`Envelope::StreamTitleChanged`]
    #[serde(rename = "stream.title.changed")]
    StreamTitleChanged,

    /// The [`EventType`] for [`Envelope::SseConnect`]
    #[serde(rename = "sse.connect")]
    SseConnect,

    /// The [`EventType`] for [`Envelope::SseDisconnect`]
    #[serde(rename = "sse.disconnect")]
    SseDisconnect,

    /// The [`EventType`] for [`Envelope::Tick`]
    #[serde(rename = "tick")]
    Tick,

    /// The [`EventType`] for [`Envelope::FediverseActivity`]
    #[serde(rename = "fediverse.activity")]
    FediverseActivity,

    /// The [`EventType`] for [`Envelope::FediverseFollow`]
    #[serde(rename = "fediverse.follow")]
    FediverseFollow,

    /// The [`EventType`] for [`Envelope::FediverseLike`]
    #[serde(rename = "fediverse.like")]
    FediverseLike,

    /// The [`EventType`] for [`Envelope::FediverseRepost`]
    #[serde(rename = "fediverse.repost")]
    FediverseRepost,

    /// The [`EventType`] for [`Envelope::FediverseQuote`]
    #[serde(rename = "fediverse.quote")]
    FediverseQuote,

    /// The [`EventType`] for [`Envelope::FediverseMention`]
    #[serde(rename = "fediverse.mention")]
    FediverseMention,

    /// The [`EventType`] for [`Envelope::FediverseReply`]
    #[serde(rename = "fediverse.reply")]
    FediverseReply,

    /// The [`EventType`] for [`Envelope::ChatCommand`]
    #[serde(rename = "chat.command")]
    ChatCommand,

    /// The [`EventType`] for [`Envelope::TimerFire`]
    #[serde(rename = "timer.fire")]
    TimerFire,

    /// A [`EventType`] for [`Envelope::Custom`]
    #[serde(untagged)]
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