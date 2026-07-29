use extism_pdk::{FromBytes, Json};
use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command_event::CommandEvent;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::fire_timer::FireTimer;
use crate::json_objects::sse_connection_event::SseConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;

#[derive(FromBytes, Deserialize)]
#[encoding(Json)]
#[serde(tag = "eventType", content = "payload")]
pub enum Envelope {
    // Chat events
    #[serde(rename = "chat.message.received")]
    ChatMessageReceived(ChatMessage),
    #[serde(rename = "chat.user.joined")]
    ChatUserJoined(User),
    #[serde(rename = "chat.user.parted")]
    ChatUserParted(User),
    #[serde(rename = "chat.user.renamed")]
    ChatUserRenamed(ChatUserRename),
    #[serde(rename = "chat.message.moderated")]
    ChatMessageModerated(ChatMessageModeration),

    // Stream lifecycle
    #[serde(rename = "stream.started")]
    StreamStarted(StreamStarted),
    #[serde(rename = "stream.stopped")]
    StreamStopped(StreamStopped),
    #[serde(rename = "stream.title.changed")]
    StreamTitleChanged(StreamTitleChange),

    // SSE connection lifecycle (who connected to / left a plugin's stream)
    #[serde(rename = "sse.connect")]
    SseConnect(SseConnectionEvent),
    #[serde(rename = "sse.disconnect")]
    SseDisconnect(SseConnectionEvent),

    // Once-a-second tick for periodic work (opt in by defining onTick)
    #[serde(rename = "tick")]
    Tick(TickEvent),

    // Fediverse, engagement (metadata only) + inbound posts (with content)
    #[serde(rename = "fediverse.activity")]
    FediverseActivity(HashMap<String, String>),
    #[serde(rename = "fediverse.follow")]
    FediverseFollow(FediverseEngagement),
    #[serde(rename = "fediverse.like")]
    FediverseLike(FediverseTargetedEngagement),
    #[serde(rename = "fediverse.repost")]
    FediverseRepost(FediverseTargetedEngagement),
    #[serde(rename = "fediverse.quote")]
    FediverseQuote(FediverseTargetedEngagement),
    #[serde(rename = "fediverse.mention")]
    FediverseMention(FediverseInboundPost),
    #[serde(rename = "fediverse.reply")]
    FediverseReply(FediverseInboundPost),

    // Internal events
    #[serde(rename = "chat.command")]
    ChatCommand(CommandEvent),
    #[serde(rename = "timer.fire")]
    TimerFire(FireTimer),

    #[serde(untagged)]
    Custom { event_type: String, payload: String }
}