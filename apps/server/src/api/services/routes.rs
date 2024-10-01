use actix_web::web;

use super::{
    core::submit_challenge, match_data::get_match_route, matches::get_matches_route,
    ping::get_api_ping,
};

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_challenge);
    cfg.service(get_api_ping);
    cfg.service(get_match_route);
    cfg.service(get_matches_route);
}
