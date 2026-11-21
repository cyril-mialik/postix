use chrono::{DateTime, Utc};
use derive_builder::{Builder, UninitializedFieldError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::{report_reason::ReportReason, report_status::ReportStatus};

pub const REPORT_MAX_LENGTH: usize = 3_000;

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
        let message = dto.message.map(|s| s.trim().to_owned());
        if message
            .as_ref()
            .is_some_and(|text| text.chars().count() > REPORT_MAX_LENGTH)
        {
            return Err(ReportError::InvalidMessageLength);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            reason: dto.reason,
            message,
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

    #[error("Текст жалобы не может превышать {REPORT_MAX_LENGTH} символов")]
    InvalidMessageLength,

    #[error("Ошибка валидации: {0}")]
    Validation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dto(message: Option<String>) -> CreateReportDto {
        CreateReportDtoBuilder::default()
            .reporter_id(Uuid::nil())
            .post_id(Uuid::from_u128(1))
            .reason(ReportReason::Spam)
            .message(message)
            .build()
            .unwrap()
    }

    #[test]
    fn trims_report_message_and_enforces_unicode_limit() {
        let valid =
            Report::new(dto(Some(format!("  {}  ", "é".repeat(REPORT_MAX_LENGTH))))).unwrap();
        assert_eq!(
            valid.message.as_deref(),
            Some("é".repeat(REPORT_MAX_LENGTH).as_str())
        );

        assert!(matches!(
            Report::new(dto(Some("é".repeat(REPORT_MAX_LENGTH + 1)))),
            Err(ReportError::InvalidMessageLength)
        ));
    }
}
