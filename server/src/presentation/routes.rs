use crate::presentation::handlers::{
    api_health::health as api_health,
    health::health,
    me::get_me,
    post::{create_post, delete_post, get_post, update_content, update_post, update_title},
    refresh::refresh,
    sign_in::sign_in,
    sign_out::sign_out,
    sign_up::sign_up,
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
        .service(api_health)
        .service(get_me)
        .service(create_post)
        .service(update_post)
        .service(update_content)
        .service(update_title)
        .service(get_post)
        .service(delete_post);
}
