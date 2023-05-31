use actix_web::web;

use super::{
    core::submit_challenge,
    match_route::get_match_route,
    ping::{get_api_ping, get_ping},
};

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_challenge);
    cfg.service(get_ping);
    cfg.service(get_api_ping);
    cfg.service(get_match_route);
}
