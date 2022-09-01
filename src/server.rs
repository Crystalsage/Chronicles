use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder, error::PathError};

use crate::Message;

#[get("/")]
async fn get_message() -> Result<Json<Message>, PathError> {
    let message = Message::new(1).pop().unwrap();
    let returnable: Json<Message> = Json(message);

    Ok(returnable)
}

pub async fn run() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(get_message)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
