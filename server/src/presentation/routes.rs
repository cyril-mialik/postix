use actix_web::web;

use super::handlers::{auth, post, profile};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::sign_up)
       .service(auth::sign_in)
       .service(auth::refresh)
       .service(auth::sign_out);
}

pub fn api_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(profile::get_me)
       .service(profile::update_profile)
       .service(post::create_post)
       .service(post::get_feed)
       .service(post::get_post)
       .service(post::update_post)
       .service(post::delete_post);
}
