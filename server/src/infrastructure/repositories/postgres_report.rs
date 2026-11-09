use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        report::{Report, ReportError},
        report_reason::ReportReason,
        report_status::ReportStatus,
    },
    repositories::report::ReportRepository,
};

pub struct PostgresReportRepository {
    pool: PgPool,
}

impl PostgresReportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReportRepository for PostgresReportRepository {
    async fn create(&self, report: &Report) -> Result<Report, Error> {
        sqlx::query!(
            r#"
            INSERT INTO report (id, reason, message, reporter_id, post_id, status_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            report.id,
            report.reason.as_str(),
            report.message,
            report.reporter_id,
            report.post_id,
            report.status.as_id(),
            report.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::Report(ReportError::AlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;
        Ok(report.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Report>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, reason, message, created_at, resolved_at, resolved_by,
                   reporter_id, post_id, status_id
            FROM report WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| Report {
            id: r.id,
            reason: ReportReason::from_str(&r.reason).unwrap_or(ReportReason::Other),
            message: r.message,
            created_at: r.created_at,
            resolved_at: r.resolved_at,
            resolved_by: r.resolved_by,
            reporter_id: r.reporter_id,
            post_id: r.post_id,
            status: ReportStatus::from_id(r.status_id).unwrap_or(ReportStatus::Pending),
        }))
    }

    async fn find_pending(&self, limit: i64, offset: i64) -> Result<Vec<Report>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, reason, message, created_at, resolved_at, resolved_by,
                   reporter_id, post_id, status_id
            FROM report WHERE status_id = 1 ORDER BY created_at ASC LIMIT $1 OFFSET $2
            "#,
            limit, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(rows
            .into_iter()
            .map(|r| Report {
                id: r.id,
                reason: ReportReason::from_str(&r.reason).unwrap_or(ReportReason::Other),
                message: r.message,
                created_at: r.created_at,
                resolved_at: r.resolved_at,
                resolved_by: r.resolved_by,
                reporter_id: r.reporter_id,
                post_id: r.post_id,
                status: ReportStatus::from_id(r.status_id).unwrap_or(ReportStatus::Pending),
            })
            .collect())
    }

    async fn resolve(
        &self,
        id: Uuid,
        resolved_by: Uuid,
        status: ReportStatus,
    ) -> Result<(), Error> {
        let result = sqlx::query!(
            "UPDATE report SET resolved_at = NOW(), resolved_by = $2, status_id = $3 WHERE id = $1",
            id, resolved_by, status.as_id()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::Report(ReportError::NotFound));
        }
        Ok(())
    }
}
