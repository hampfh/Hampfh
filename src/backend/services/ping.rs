use actix_web::get;

#[get("/")]
pub(super) async fn get_ping() -> actix_web::Result<String> {
    return Ok(format!("Hi there, welcome to the github profile project!\nGoto my github profile (@hampfh) and start writing your own bot!"));
}
