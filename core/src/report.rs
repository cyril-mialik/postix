use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportReason {
    Spam,
    Abuse,
    Nsfw,
    Violence,
    Copyright,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreateReportRequest {
    pub reason: ReportReason,
    pub message: Option<String>,
}
