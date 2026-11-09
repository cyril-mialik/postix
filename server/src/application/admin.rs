use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        report::Report,
        report_status::ReportStatus,
        user::UserError,
    },
    repositories::{report::ReportRepository, token::TokenRepository, user::UserRepository},
};

#[derive(Clone)]
pub struct AdminService {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn TokenRepository>,
    report_repo: Arc<dyn ReportRepository>,
}

impl AdminService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn TokenRepository>,
        report_repo: Arc<dyn ReportRepository>,
    ) -> Self {
        Self { user_repo, token_repo, report_repo }
    }

    pub async fn ban_user(&self, admin_id: Uuid, user_id: Uuid, reason: &str) -> Result<(), Error> {
        let admin = self
            .user_repo
            .find_by_id(admin_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;
        admin.ensure_can_moderate()?;

        self.user_repo.ban(user_id, reason).await?;
        self.token_repo.revoke_all_refresh_tokens(user_id).await?;

        Ok(())
    }

    pub async fn unban_user(&self, admin_id: Uuid, user_id: Uuid) -> Result<(), Error> {
        let admin = self
            .user_repo
            .find_by_id(admin_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;
        admin.ensure_can_moderate()?;

        self.user_repo.unban(user_id).await
    }

    pub async fn get_pending_reports(
        &self,
        admin_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Report>, Error> {
        let admin = self
            .user_repo
            .find_by_id(admin_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;
        admin.ensure_can_moderate()?;

        self.report_repo.find_pending(limit, offset).await
    }

    pub async fn resolve_report(
        &self,
        admin_id: Uuid,
        report_id: Uuid,
        status: ReportStatus,
    ) -> Result<(), Error> {
        let admin = self
            .user_repo
            .find_by_id(admin_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;
        admin.ensure_can_moderate()?;

        self.report_repo.resolve(report_id, admin_id, status).await
    }
}
