use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Pending,
    ResolvedDismissed,
    ResolvedActioned,
}

impl ReportStatus {
    const PENDING_IDX: i32 = 1;
    const RESOLVED_DISMISSED_IDX: i32 = 2;
    const RESOLVED_ACTIONED_IDX: i32 = 3;

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            Self::PENDING_IDX => Some(ReportStatus::Pending),
            Self::RESOLVED_DISMISSED_IDX => Some(ReportStatus::ResolvedDismissed),
            Self::RESOLVED_ACTIONED_IDX => Some(ReportStatus::ResolvedActioned),
            _ => None,
        }
    }

    pub fn as_id(&self) -> i32 {
        match self {
            ReportStatus::Pending => Self::PENDING_IDX,
            ReportStatus::ResolvedDismissed => Self::RESOLVED_DISMISSED_IDX,
            ReportStatus::ResolvedActioned => Self::RESOLVED_ACTIONED_IDX,
        }
    }

    pub fn is_peding(&self) -> bool {
        matches!(self, ReportStatus::Pending)
    }

    pub fn is_resolved_dismissed(&self) -> bool {
        matches!(self, ReportStatus::ResolvedDismissed)
    }

    pub fn is_resolved_actioned(&self) -> bool {
        matches!(self, ReportStatus::ResolvedActioned)
    }
}
