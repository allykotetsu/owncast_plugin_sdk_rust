use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) enum Event {
    // Chat events
    #[serde(rename(serialize = "chat.message.received"))]
    ChatMessageReceived,
    #[serde(rename(serialize = "chat.user.joined"))]
    ChatUserJoined,
    #[serde(rename(serialize = "chat.user.parted"))]
    ChatUserParted,
    #[serde(rename(serialize = "chat.user.renamed"))]
    ChatUserRenamed,
    #[serde(rename(serialize = "chat.message.moderated"))]
    ChatMessageModerated,

    // Stream lifecycle
    #[serde(rename(serialize = "stream.started"))]
    StreamStarted,
    #[serde(rename(serialize = "stream.stopped"))]
    StreamStopped,
    #[serde(rename(serialize = "stream.title.changed"))]
    StreamTitleChanged,

    // SSE connection lifecycle (who connected to / left a plugin's stream)
    #[serde(rename(serialize = "sse.connect"))]
    SseConnect,
    #[serde(rename(serialize = "sse.disconnect"))]
    SseDisconnect,

    // Once-a-second tick for periodic work (opt in by defining onTick)
    #[serde(rename(serialize = "tick"))]
    Tick,

    // Fediverse, engagement (metadata only) + inbound posts (with content)
    #[serde(rename(serialize = "fediverse.follow"))]
    FediverseFollow,
    #[serde(rename(serialize = "fediverse.like"))]
    FediverseLike,
    #[serde(rename(serialize = "fediverse.repost"))]
    FediverseRepost,
    #[serde(rename(serialize = "fediverse.mention"))]
    FediverseMention,
    #[serde(rename(serialize = "fediverse.reply"))]
    FediverseReply,

    // TODO serialize and deserialize unmatched names as Custom
    Custom(String)
}