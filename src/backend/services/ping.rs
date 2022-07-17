use actix_web::get;

#[get("/api/ping")]
pub(super) async fn get_ping() -> actix_web::Result<String> {
    return Ok(format!("Service is up!"));
}
