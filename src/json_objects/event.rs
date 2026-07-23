use extism_pdk::{FromBytes, Json};
use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde_json::Error as JsonError;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command_event::CommandEvent;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::sse_connection_event::SSEConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;
use crate::prelude::EventType;

#[derive(FromBytes, Deserialize)]
#[encoding(Json)]
#[serde(tag = "event_type")]
pub enum Event {
    // Chat events
    #[serde(rename = "chat.message.received")]
    ChatMessageReceived { payload: ChatMessage },
    #[serde(rename = "chat.user.joined")]
    ChatUserJoined { payload: User },
    #[serde(rename = "chat.user.parted")]
    ChatUserParted { payload: User },
    #[serde(rename = "chat.user.renamed")]
    ChatUserRenamed { payload: ChatUserRename },
    #[serde(rename = "chat.message.moderated")]
    ChatMessageModerated { payload: ChatMessageModeration },

    // Stream lifecycle
    #[serde(rename = "stream.started")]
    StreamStarted { payload: StreamStarted },
    #[serde(rename = "stream.stopped")]
    StreamStopped { payload: StreamStopped },
    #[serde(rename = "stream.title.changed")]
    StreamTitleChanged { payload: StreamTitleChange },

    // SSE connection lifecycle (who connected to / left a plugin's stream)
    #[serde(rename = "sse.connect")]
    SseConnect { payload: SSEConnectionEvent },
    #[serde(rename = "sse.disconnect")]
    SseDisconnect { payload: SSEConnectionEvent },

    // Once-a-second tick for periodic work (opt in by defining onTick)
    #[serde(rename = "tick")]
    Tick { payload: TickEvent },

    // Fediverse, engagement (metadata only) + inbound posts (with content)
    #[serde(rename = "fediverse.activity")]
    FediverseActivity { payload: HashMap<String, String> },
    #[serde(rename = "fediverse.follow")]
    FediverseFollow { payload: FediverseEngagement },
    #[serde(rename = "fediverse.like")]
    FediverseLike { payload: FediverseTargetedEngagement },
    #[serde(rename = "fediverse.repost")]
    FediverseRepost { payload: FediverseTargetedEngagement },
    #[serde(rename = "fediverse.quote")]
    FediverseQuote { payload: FediverseTargetedEngagement },
    #[serde(rename = "fediverse.mention")]
    FediverseMention { payload: FediverseInboundPost },
    #[serde(rename = "fediverse.reply")]
    FediverseReply { payload: FediverseInboundPost },

    // Internal events
    #[serde(rename = "chat.command")]
    ChatCommand { payload: CommandEvent },
    #[serde(rename = "timer.fire")]
    TimerFire{ payload: () },

    #[serde(untagged)]
    Custom { event_type: String, payload: String }
}