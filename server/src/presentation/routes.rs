use crate::presentation::handlers::{
    auth::{refresh, sign_in, sign_out, sign_up},
    health::health,
    me::get_me,
    post::{
        create_post, delete_post, get_post_by_id, get_post_by_user_id, update_content, update_post,
        update_title,
    },
    settings::{
        create_settings, delete_settings, update_first_name, update_last_name, update_settings,
        update_username,
    },
};

pub fn public_configure(config: &mut actix_web::web::ServiceConfig) {
    config.service(health);
}

pub fn auth_configure(config: &mut actix_web::web::ServiceConfig) {
    config.service(sign_up);
    config.service(sign_in);
    config.service(refresh);
    config.service(sign_out);
}

pub fn api_configure(config: &mut actix_web::web::ServiceConfig) {
    config
        .service(health)
        .service(get_me)
        .service(create_post)
        .service(update_post)
        .service(update_content)
        .service(update_title)
        .service(get_post_by_id)
        .service(get_post_by_user_id)
        .service(delete_post)
        .service(create_settings)
        .service(update_first_name)
        .service(update_last_name)
        .service(delete_settings)
        .service(update_settings)
        .service(update_username);
}
