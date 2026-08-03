use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActionButton {
    pub title: String,
    pub url: Option<String>,
    pub html: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub open_externally: Option<bool>
}

impl ActionButton {
    pub fn url(title: &str, url: &str) -> Self {
        Self {
            title: title.to_string(),
            url: Some(url.to_string()),
            html: None,
            icon: None,
            color: None,
            description: None,
            open_externally: None,
        }
    }

    pub fn html(title: &str, html: &str) -> Self {
        Self {
            title: title.to_string(),
            url: None,
            html: Some(html.to_string()),
            icon: None,
            color: None,
            description: None,
            open_externally: None,
        }
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    // TODO restrict color passed
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_open_externally(mut self) -> Self {
        self.open_externally = Some(true);
        self
    }
}

impl From<ActionButton> for Vec<ActionButton> {
    fn from(action_button: ActionButton) -> Self {
        vec![action_button]
    }
}