use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::PostError,
        report::{CreateReportDtoBuilder, Report, ReportError},
        report_reason::ReportReason,
    },
    repositories::{post::PostRepository, report::ReportRepository},
};

#[derive(Clone)]
pub struct ReportService {
    report_repo: Arc<dyn ReportRepository>,
    post_repo: Arc<dyn PostRepository>,
}

impl ReportService {
    pub fn new(report_repo: Arc<dyn ReportRepository>, post_repo: Arc<dyn PostRepository>) -> Self {
        Self {
            report_repo,
            post_repo,
        }
    }

    pub async fn create_report(
        &self,
        reporter_id: Uuid,
        post_id: Uuid,
        reason: ReportReason,
        message: Option<String>,
    ) -> Result<Report, Error> {
        self.post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        let dto = CreateReportDtoBuilder::default()
            .reporter_id(reporter_id)
            .post_id(post_id)
            .reason(reason)
            .message(message)
            .build()
            .map_err(|e| Error::Report(ReportError::Validation(e.to_string())))?;

        let report = Report::new(dto)?;
        self.report_repo.create(&report).await
    }
}
