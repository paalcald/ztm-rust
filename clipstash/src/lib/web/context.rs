use derive_more::Constructor;
use serde::Serialize;
use sqlx::sqlite::SqliteValueRef;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for Home {
    fn title(&self) -> &str {
        "Stash Your Clipboard!"
    }

    fn template_path(&self) -> &str {
        "home"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Constructor, Debug, Serialize)]
pub struct ViewClip {
    pub clip: crate::domain::clip::Clip,
}

impl PageContext for ViewClip {
    fn title(&self) -> &str {
        "View Clip"
    }

    fn template_path(&self) -> &str {
        "clip"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Constructor, Debug, Serialize)]
pub struct PasswordRequired {
    shortcode: crate::domain::clip::field::ShortCode,
}

impl PageContext for PasswordRequired {
    fn title(&self) -> &str {
        "Password Required"
    }

    fn template_path(&self) -> &str {
        "clip_need_password"
    }
    fn parent(&self) -> &str {
        "base"
    }
}
