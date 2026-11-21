use actix_web::{HttpResponse, post, web};
use postix_core::report::{CreateReportRequest, ReportReason};
use uuid::Uuid;

use crate::{
    application::report::ReportService,
    domain::entities::{errors::Error, report_reason::ReportReason as DomainReportReason},
    presentation::extractors::jwt_auth::AuthenticatedUser,
};

#[post("/posts/{id}/reports")]
pub async fn create_report(
    auth: AuthenticatedUser,
    service: web::Data<ReportService>,
    path: web::Path<Uuid>,
    payload: web::Json<CreateReportRequest>,
) -> Result<HttpResponse, Error> {
    let reason = match payload.reason {
        ReportReason::Spam => DomainReportReason::Spam,
        ReportReason::Abuse => DomainReportReason::Abuse,
        ReportReason::Nsfw => DomainReportReason::Nsfw,
        ReportReason::Violence => DomainReportReason::Violence,
        ReportReason::Copyright => DomainReportReason::Copyright,
        ReportReason::Other => DomainReportReason::Other,
    };
    service
        .create_report(
            auth.user_id,
            path.into_inner(),
            reason,
            payload.message.clone(),
        )
        .await?;
    Ok(HttpResponse::Created().finish())
}
