use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
}

impl MediaType {
    const IMAGE: &str = "image";
    const VIDEO: &str = "video";

    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Image => Self::IMAGE,
            MediaType::Video => Self::VIDEO,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, MediaType::Image)
    }

    pub fn is_video(&self) -> bool {
        matches!(self, MediaType::Video)
    }
}
