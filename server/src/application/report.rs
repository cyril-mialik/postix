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
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible()?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entities::{
            post::{CreatePostDtoBuilder, Post},
            post_status::PostStatus,
            report_reason::ReportReason,
        },
        repositories::{post::MockPostRepository, report::MockReportRepository},
    };

    #[actix_web::test]
    async fn reports_can_only_be_created_for_public_posts() {
        let reporter_id = Uuid::new_v4();
        let mut post = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(Uuid::new_v4())
                .title("A public story".to_owned())
                .content(serde_json::json!({"text": "Story"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        post.status = PostStatus::Published;
        let post_id = post.id;

        let mut posts = MockPostRepository::new();
        posts
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post.clone())));
        let mut reports = MockReportRepository::new();
        reports
            .expect_create()
            .withf(move |report| report.reporter_id == reporter_id && report.post_id == post_id)
            .once()
            .returning(|report| Ok(report.clone()));

        ReportService::new(Arc::new(reports), Arc::new(posts))
            .create_report(reporter_id, post_id, ReportReason::Spam, None)
            .await
            .unwrap();
    }
}
