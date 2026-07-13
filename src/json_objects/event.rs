use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::sse_connection_event::SSEConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;

#[derive(Deserialize)]
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
    SseConnect(SSEConnectionEvent),
    SseDisconnect(SSEConnectionEvent),

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