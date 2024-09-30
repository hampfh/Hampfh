use actix_web::get;

#[get("/api/ping")]
pub(super) async fn get_api_ping() -> actix_web::Result<String> {
    return Ok("pong".to_string());
}
