use serde::Deserialize;

/// [`Envelope::TimerFire`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimerFireEvent {
    pub(crate) id: i64
}