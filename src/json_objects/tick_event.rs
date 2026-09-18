use serde::Deserialize;

/// [`Envelope::Tick`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickEvent {
    /// The current time.in UNIX milliseconds.
    pub now: i64,
}