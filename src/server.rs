use std::num;
use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{get, web::Json, App, HttpServer, error::PathError};
use redis::{Client, Commands};
use redis::RedisError;
use redis::RedisResult;

use serde::{Serialize, Deserialize};

use crate::{Message, Post, Platform};

extern crate redis;


// Redis notes:
// Table 0 is for posts
// table 1 is for messages
//


mod Signals {
    enum PostSignal {
        Success,
        Failure
    }

    struct PostError;
    struct MessageError;
}

#[get("/create_post")]
async fn create_post(data: Data<Mutex<redis::Connection>>) -> Result<String, PathError> {
    let post: Post = Post {
        id: 1,
        platform: Platform::IRC,
        messages: get_messages(),
    };

    let post = serde_json::to_string(&post).unwrap();
    let mut con = data.lock().unwrap();
    con.set::<String, String, String> ("1".to_string(), post);

    Ok(String::from("0"))
}


#[get("/")]
async fn hello_there() -> Result<String, PathError> {
    let returnable = String::from("Hello there this is the actix server!");
    Ok(returnable)
}

fn get_messages() -> Vec<Message> {
    return Message::new(5);
}

pub async fn run() -> std::io::Result<()>{
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    let con = Data::new(Mutex::new(con));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&con))
            .service(hello_there)
            .service(create_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
