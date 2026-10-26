use std::sync::Arc;

use crate::{
    application::{auth::AuthService, post::PostService, user::UserService},
    infrastructure::{
        cache::redis::create_redis_pool,
        config::env::Config,
        database::{migrate, postgres::create_postgres_pool},
        repositories::{
            post::PostgresPostRepository, token::RedisTokenRepository, user::PostgresUserRepository,
        },
    },
    presentation::middleware::auth_middleware::AuthMiddleware,
};

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env().expect("Can't read the config file.");

    let postgres_pool = create_postgres_pool(&cfg.database_url).await;
    let redis_pool = create_redis_pool(&cfg.cache_url);

    migrate::run(&postgres_pool)
        .await
        .expect("Database migration was failed.");

    let user_repository = Arc::new(PostgresUserRepository::new(postgres_pool.clone()));
    let token_repository = Arc::new(RedisTokenRepository::new(redis_pool.clone()));
    let post_repository = Arc::new(PostgresPostRepository::new(postgres_pool.clone()));

    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let post_service = Arc::new(PostService::new(post_repository.clone()));
    let auth_service = Arc::new(AuthService::new(
        user_repository,
        token_repository,
        cfg.jwt_secret.clone(),
    ));

    let addr = format!("{}:{}", cfg.host, cfg.port);
    println!("The server is listening on http://{}", addr);

    let cors_origin = cfg.cors_origin.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::from(user_service.clone()))
            .app_data(web::Data::from(auth_service.clone()))
            .app_data(web::Data::from(post_service.clone()))
            .configure(presentation::routes::public_configure)
            .service(web::scope("/auth").configure(presentation::routes::auth_configure))
            .service(
                web::scope("/api")
                    .wrap(AuthMiddleware)
                    .configure(presentation::routes::api_configure),
            )
    })
    .bind(addr)?
    .run()
    .await
}
