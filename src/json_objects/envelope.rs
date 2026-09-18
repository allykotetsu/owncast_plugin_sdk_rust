use extism_pdk::{FromBytes, Json};
use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command_event::CommandEvent;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_quote::FediverseQuote;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::timer_fire_event::TimerFireEvent;
use crate::json_objects::sse_connection_event::SseConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;

/// The event type and payload being sent when an event happens.
#[derive(FromBytes, Deserialize, Clone, Debug)]
#[encoding(Json)]
#[serde(tag = "eventType", content = "payload")]
pub enum Envelope {
    /// The event [`Envelope`] for when a [`ChatMessage`] is sent.
    #[serde(rename = "chat.message.received")]
    ChatMessageReceived(ChatMessage),

    /// The event [`Envelope`] for when a [`User`] joins stream.
    #[serde(rename = "chat.user.joined")]
    ChatUserJoined(User),

    /// The event [`Envelope`] for when a [`User`] leaves stream.
    #[serde(rename = "chat.user.parted")]
    ChatUserParted(User),

    /// The event [`Envelope`] for when a [`User`] is renamed.
    #[serde(rename = "chat.user.renamed")]
    ChatUserRenamed(ChatUserRename),

    /// The event [`Envelope`] for when a [`ChatMessage`] is moderated.
    #[serde(rename = "chat.message.moderated")]
    ChatMessageModerated(ChatMessageModeration),

    /// The event [`Envelope`] for when the stream starts.
    #[serde(rename = "stream.started")]
    StreamStarted(StreamStarted),

    /// The event [`Envelope`] for when the stream stops.
    #[serde(rename = "stream.stopped")]
    StreamStopped(StreamStopped),

    /// The event [`Envelope`] for when the stream title changes.
    #[serde(rename = "stream.title.changed")]
    StreamTitleChanged(StreamTitleChange),

    /// The event [`Envelope`] for when a user connects to SSE.
    #[serde(rename = "sse.connect")]
    SseConnect(SseConnectionEvent),

    /// The event [`Envelope`] for when a user disconnects from SSE.
    #[serde(rename = "sse.disconnect")]
    SseDisconnect(SseConnectionEvent),

    /// The event [`Envelope`] for once-a-second ticks.
    #[serde(rename = "tick")]
    Tick(TickEvent),

    /// The event [`Envelope`] for when any fediverse activity happens.
    #[serde(rename = "fediverse.activity")]
    FediverseActivity(HashMap<String, String>),

    /// The event [`Envelope`] for when a user follow the streamer on the fediverse.
    #[serde(rename = "fediverse.follow")]
    FediverseFollow(FediverseEngagement),

    /// The event [`Envelope`] for when a user likes a fediverse post.
    #[serde(rename = "fediverse.like")]
    FediverseLike(FediverseTargetedEngagement),

    /// The event [`Envelope`] for when a user reposts a fediverse post.
    #[serde(rename = "fediverse.repost")]
    FediverseRepost(FediverseTargetedEngagement),

    /// The event [`Envelope`] for when a user quotes a fediverse post.
    #[serde(rename = "fediverse.quote")]
    FediverseQuote(FediverseQuote),

    /// The event [`Envelope`] for when a user mentions the streamer on the fediverse.
    #[serde(rename = "fediverse.mention")]
    FediverseMention(FediverseInboundPost),

    /// The event [`Envelope`] for when a user replies to a fediverse post.
    #[serde(rename = "fediverse.reply")]
    FediverseReply(FediverseInboundPost),

    /// The event [`Envelope`] for when a [`User`] in chat uses a chat command.
    #[serde(rename = "chat.command")]
    ChatCommand(CommandEvent),

    /// The event [`Envelope`] for when Owncast triggers a plugin-set timer.
    #[serde(rename = "timer.fire")]
    TimerFire(TimerFireEvent),

    /// A custom event.
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    Custom {
        /// The unique name of the [`Envelope::Custom`] event.
        event_type: String,

        /// The payload of the [`Envelope::Custom`] event in String form.
        payload: String
    }
}