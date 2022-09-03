use std::num;

use actix_web::{get, web::Json, App, HttpServer, error::PathError};

use crate::{Message, Post, Platform};

extern crate redis;
use redis::Commands;
use redis::RedisError;

mod errors {
    struct PostError;
    struct MessageError;
}

fn fetch_integer() -> redis::RedisResult<u8> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con: redis::Connection = client.get_connection()?;

    let _: Result<u8, RedisError> = redis::cmd("SELECT").arg("1").query(&mut con);
    let numero: u8 = con.get("numero")?;

    Ok(numero)
}


#[get("/redis")]
async fn get_redis_integer() -> Result<String, PathError>{
    let num = fetch_integer().unwrap();

    Ok(num.to_string())
}

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
            .service(get_redis_integer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
