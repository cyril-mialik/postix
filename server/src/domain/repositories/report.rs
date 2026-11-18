use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, report::Report, report_status::ReportStatus};

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn create(&self, report: &Report) -> Result<Report, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Report>, Error>;
    async fn find_pending(&self, limit: i64, offset: i64) -> Result<Vec<Report>, Error>;
    async fn resolve(&self, id: Uuid, resolved_by: Uuid, status: ReportStatus)
    -> Result<(), Error>;
}
