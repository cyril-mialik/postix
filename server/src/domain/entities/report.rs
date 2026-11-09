use chrono::{DateTime, Utc};
use derive_builder::{Builder, UninitializedFieldError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::{report_reason::ReportReason, report_status::ReportStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub reason: ReportReason,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub reporter_id: Uuid,
    pub post_id: Uuid,
    pub status: ReportStatus,
}

impl Report {
    pub fn new(dto: CreateReportDto) -> Result<Self, ReportError> {
        Ok(Self {
            id: Uuid::new_v4(),
            reason: dto.reason,
            message: dto.message.map(|s| s.trim().to_owned()),
            created_at: Utc::now(),
            resolved_at: None,
            resolved_by: None,
            reporter_id: dto.reporter_id,
            post_id: dto.post_id,
            status: ReportStatus::Pending,
        })
    }

    pub fn resolve(&mut self, resolved_by: Uuid, status: ReportStatus) {
        self.resolved_at = Some(Utc::now());
        self.resolved_by = Some(resolved_by);
        self.status = status;
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into), build_fn(error = "CreateReportDtoBuilderError"))]
pub struct CreateReportDto {
    pub reporter_id: Uuid,
    pub post_id: Uuid,
    pub reason: ReportReason,
    #[builder(default)]
    pub message: Option<String>,
}

#[derive(Debug, Error)]
pub enum CreateReportDtoBuilderError {
    #[error(transparent)]
    UninitializedField(#[from] UninitializedFieldError),
}

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Репорт не найден")]
    NotFound,

    #[error("Вы уже отправили репорт на этот пост")]
    AlreadyExists,

    #[error("Ошибка валидации: {0}")]
    Validation(String),
}
