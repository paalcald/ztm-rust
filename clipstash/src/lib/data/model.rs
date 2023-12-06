use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};

use crate::domain::clip::field::ShortCode;

pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = crate::domain::clip::ClipError;
    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::data::DbId;
        use crate::domain::clip::field::*;
        use crate::domain::time::Time;
        Ok(Self {
            clip_id: ClipId::new(DbId::from_str(&value.clip_id)?),
            shortcode: ShortCode::from_str(&value.shortcode)?,
            content: Content::new(&value.content)?,
            title: Title::new(value.title),
            posted: Posted::new(Time::from_naive_utc(value.posted)),
            expires: Expires::new(value.expires.map(Time::from_naive_utc)),
            password: Password::new(value.password)?,
            hits: Hits::new(value.hits),
        })
    }
}

pub struct GetClip {
    pub(in crate::data) shortcode: String,
}
impl From<crate::service::ask::GetClip> for GetClip {
    fn from(value: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
        }
    }
}
impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        GetClip {
            shortcode: value.into_inner(),
        }
    }
}
impl From<String> for GetClip {
    fn from(value: String) -> Self {
        GetClip { shortcode: value }
    }
}

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::NewClip> for NewClip {
    fn from(value: crate::service::ask::NewClip) -> Self {
        use crate::data::DbId;
        Self {
            clip_id: DbId::new().into(),
            shortcode: ShortCode::default().into(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            posted: Utc::now().timestamp(),
            expires: value.expires.into_inner().map(|time| time.timestamp()),
            password: value.password.into_inner(),
        }
    }
}

pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::UpdateClip> for UpdateClip {
    fn from(value: crate::service::ask::UpdateClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            expires: value.expires.into_inner().map(|time| time.timestamp()),
            password: value.password.into_inner(),
        }
    }
}

pub struct IncreaseHitCount {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) hits: i64,
}

impl From<crate::service::ask::IncreaseHitCount> for IncreaseHitCount {
    fn from(value: crate::service::ask::IncreaseHitCount) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
            hits: value.hits.into_inner(),
        }
    }
}
