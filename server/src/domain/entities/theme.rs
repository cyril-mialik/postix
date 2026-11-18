use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Theme {
    const LIGTH_IDX: i32 = 1;
    const DARK_IDX: i32 = 2;
    const AUTO_IDX: i32 = 3;

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            Self::LIGTH_IDX => Some(Theme::Light),
            Self::DARK_IDX => Some(Theme::Dark),
            Self::AUTO_IDX => Some(Theme::Auto),
            _ => None,
        }
    }

    pub fn as_id(&self) -> i32 {
        match self {
            Theme::Light => Self::LIGTH_IDX,
            Theme::Dark => Self::DARK_IDX,
            Theme::Auto => Self::AUTO_IDX,
        }
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, Theme::Dark)
    }

    pub fn is_light(&self) -> bool {
        matches!(self, Theme::Light)
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, Theme::Auto)
    }
}
