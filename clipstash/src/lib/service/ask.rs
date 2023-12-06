use crate::domain::clip::{
    field::{self, Password, ShortCode},
    ClipError,
};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use std::{default, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: Password,
}
impl FromStr for GetClip {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            shortcode: ShortCode::from_str(s)?,
            password: Password::default(),
        })
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(value: &str) -> Self {
        Self {
            shortcode: ShortCode::from(value),
            password: Password::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClip {
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncreaseHitCount {
    pub shortcode: field::ShortCode,
    pub hits: field::Hits,
}
