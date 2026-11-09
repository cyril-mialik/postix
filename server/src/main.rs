mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use infrastructure::{config::Config, migration, redis};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::application::{
    admin::AdminService, auth::AuthService, bookmark::BookmarkService, follow::FollowService,
    notification::NotificationService, post::PostService, profile::ProfileService,
    rating::RatingService, report::ReportService, review::ReviewService,
};
use crate::domain::repositories::post::PostRepository;
use crate::domain::repositories::{
    bookmark::BookmarkRepository, follow::FollowRepository, notification::NotificationRepository,
    rating::RatingRepository, report::ReportRepository, review::ReviewRepository,
    tag::TagRepository, token::TokenRepository, user::UserRepository,
};
use crate::infrastructure::repositories::postgres_post::PostgresPostRepository;
use crate::infrastructure::{
    middleware::auth::JwtAuth,
    repositories::{
        postgres_bookmark::PostgresBookmarkRepository, postgres_follow::PostgresFollowRepository,
        postgres_notification::PostgresNotificationRepository,
        postgres_rating::PostgresRatingRepository, postgres_report::PostgresReportRepository,
        postgres_review::PostgresReviewRepository, postgres_tag::PostgresTagRepository,
        postgres_user::PostgresUserRepository, redis_token::RedisTokenRepository,
    },
};
use crate::presentation::routes::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env().expect("invalid config");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    migration::run(&pool).await.expect("migrations failed");

    let cache = redis::create_pool(&cfg.redis_url);

    let user_repo: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(pool.clone()));
    let tag_repo: Arc<dyn TagRepository> = Arc::new(PostgresTagRepository::new(pool.clone()));
    let rating_repo: Arc<dyn RatingRepository> =
        Arc::new(PostgresRatingRepository::new(pool.clone()));
    let review_repo: Arc<dyn ReviewRepository> =
        Arc::new(PostgresReviewRepository::new(pool.clone()));
    let follow_repo: Arc<dyn FollowRepository> =
        Arc::new(PostgresFollowRepository::new(pool.clone()));
    let bookmark_repo: Arc<dyn BookmarkRepository> =
        Arc::new(PostgresBookmarkRepository::new(pool.clone()));
    let notification_repo: Arc<dyn NotificationRepository> =
        Arc::new(PostgresNotificationRepository::new(pool.clone()));
    let report_repo: Arc<dyn ReportRepository> =
        Arc::new(PostgresReportRepository::new(pool.clone()));
    let token_repo: Arc<dyn TokenRepository> = Arc::new(RedisTokenRepository::new(cache));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PostgresPostRepository::new(pool.clone()));

    let auth_service = AuthService::new(
        user_repo.clone(),
        token_repo.clone(),
        cfg.jwt_secret.clone(),
    );
    let profile_service = ProfileService::new(user_repo.clone(), follow_repo.clone());
    let post_service = PostService::new(post_repo.clone(), tag_repo.clone(), user_repo.clone());
    let rating_service = RatingService::new(rating_repo, post_repo.clone());
    let review_service = ReviewService::new(review_repo, post_repo.clone());
    let follow_service = FollowService::new(follow_repo, user_repo.clone());
    let bookmark_service = BookmarkService::new(bookmark_repo, post_repo.clone());
    let notification_service = NotificationService::new(notification_repo);
    let report_service = ReportService::new(report_repo.clone(), post_repo);
    let admin_service = AdminService::new(user_repo, token_repo, report_repo);

    let addr = format!("{}:{}", cfg.host, cfg.port);
    println!("listening on http://{}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(profile_service.clone()))
            .app_data(web::Data::new(post_service.clone()))
            .app_data(web::Data::new(rating_service.clone()))
            .app_data(web::Data::new(review_service.clone()))
            .app_data(web::Data::new(follow_service.clone()))
            .app_data(web::Data::new(bookmark_service.clone()))
            .app_data(web::Data::new(notification_service.clone()))
            .app_data(web::Data::new(report_service.clone()))
            .app_data(web::Data::new(admin_service.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .configure(presentation::routes::health_configure)
            .configure(presentation::routes::configure)
            .service(
                web::scope("/api")
                    .wrap(JwtAuth::new(cfg.jwt_secret.clone()))
                    .configure(presentation::routes::api_configure),
            )
    })
    .bind(addr)?
    .run()
    .await
}
