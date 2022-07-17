use actix_web::web;

use super::{core::submit_challenge, ping::get_ping};

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_challenge);
    cfg.service(get_ping);
}
