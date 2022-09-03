use actix_web::{get, web::Json, App, HttpServer, error::PathError};

use crate::{Message, Post, Platform};

#[get("/")]
async fn hello_there() -> Result<String, PathError> {
    let returnable = String::from("Hello there this is the actix server!");
    Ok(returnable)
}

fn get_messages() -> Vec<Message> {
    return Message::new(5);
}

#[get("/create_post")]
async fn create_post() -> Result<Json<Post>, PathError> {
    let post: Post = Post {
        id: 1,
        platform: Platform::IRC,
        messages: get_messages(),
    };

    Ok(Json(post))
}

pub async fn run() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hello_there)
            .service(create_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
