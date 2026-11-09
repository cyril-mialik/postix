use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportReason {
    Spam,
    Abuse,
    Nsfw,
    Violence,
    Copyright,
    Other,
}

impl ReportReason {
    const SPAM: &str = "spam";
    const ABUSE: &str = "abuse";
    const NSFW: &str = "nsfw";
    const VIOLENCE: &str = "violence";
    const COPYRIGHT: &str = "copyright";
    const OTHER: &str = "other";

    pub fn as_str(&self) -> &'static str {
        match self {
            ReportReason::Spam => Self::SPAM,
            ReportReason::Abuse => Self::ABUSE,
            ReportReason::Nsfw => Self::NSFW,
            ReportReason::Violence => Self::VIOLENCE,
            ReportReason::Copyright => Self::COPYRIGHT,
            ReportReason::Other => Self::OTHER,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            Self::SPAM => Some(ReportReason::Spam),
            Self::ABUSE => Some(ReportReason::Abuse),
            Self::NSFW => Some(ReportReason::Nsfw),
            Self::VIOLENCE => Some(ReportReason::Violence),
            Self::COPYRIGHT => Some(ReportReason::Copyright),
            Self::OTHER => Some(ReportReason::Other),
            _ => None,
        }
    }

    pub fn is_spam(&self) -> bool {
        matches!(self, ReportReason::Spam)
    }

    pub fn is_abuse(&self) -> bool {
        matches!(self, ReportReason::Abuse)
    }

    pub fn is_nsfw(&self) -> bool {
        matches!(self, ReportReason::Nsfw)
    }

    pub fn is_violence(&self) -> bool {
        matches!(self, ReportReason::Violence)
    }

    pub fn is_copyright(&self) -> bool {
        matches!(self, ReportReason::Copyright)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, ReportReason::Other)
    }
}
