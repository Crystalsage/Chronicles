use core::fmt;
use std::str::from_utf8;
use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{get, web,  post, App, HttpServer, error::PathError};
use redis::Commands;

use actix_cors::Cors;

use crate::{Message, Post, DiscordMessages};


use actix_web::Responder;

extern crate redis;

// Redis notes:
// Table 0 is for posts
// table 1 is for messages
#[derive(Debug, serde::Deserialize)]
pub struct PostId {
    id: u8,
}

#[derive(Debug)]
pub struct PostError;

type JsonString = std::string::String;

// ==========================================================
// Post construction endpoints and helpers below this comment
// ==========================================================

#[post("/create_post")]
async fn create_post(data: Data<Mutex<redis::Connection>>, msgs: web::Bytes) -> Result<String, PathError> {
    let msgs_from_discord: Vec<Message> = convert_discord_to_crate_type(msgs).await.unwrap();

    let post: Post = Post::new(msgs_from_discord);
    let post_json = serde_json::to_string(&post).unwrap();

    let mut con = data.lock().unwrap();

    println!("Committing post to Redis!");

    con.set::<String, String, String>(post.id.to_string(), post_json)
        .expect("Redis SET failed for POST");

    return Ok("Commited post".to_string());
}

#[get("/get_post_by_id")]
async fn get_post_by_id(post_id: web::Query<PostId>, data: Data<Mutex<redis::Connection>>) -> JsonString {
    println!("Fetching post with the id: {}", post_id.id);

    let mut con = data.lock().unwrap();

    let post_json: String = con.get(post_id.id)
        .unwrap();

    return post_json;
}


async fn convert_discord_to_crate_type(msgs: web::Bytes) -> Result<Vec<Message>, PostError>{
    let mut messages: Vec<Message> = Vec::new();

    let discord_type_messages: &str  = from_utf8(&msgs).unwrap();
    let discord_type_messages: DiscordMessages = serde_json::from_str(discord_type_messages).unwrap();

    for discord_message in discord_type_messages.messages {
        messages.push(Message { 
            timestamp: 13909203, 
            content: discord_message.to_string(), 
            author: "Bourbon".to_string(), 
        })
    }

    Ok(messages)
}



#[get("/")]
async fn root_hello() -> Result<String, PathError> {
    println!("We got a hit at hello_there!");
    let returnable = String::from("Hello there this is the actix server!");
    Ok(returnable)
}

pub async fn run() -> std::io::Result<()>{
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().expect("Failed to connect to Redis!");

    println!("Starting server!");

    let con = Data::new(Mutex::new(con));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&con))
            .wrap(
                Cors::default()
                .allow_any_origin()
                .send_wildcard())
                //.allowed_origin("0.0.0.0:8080")
                //.allowed_methods(vec!["GET", "POST"])
                //.max_age(3600))
            .service(root_hello)
            .service(create_post)
            .service(get_post_by_id)
    })
    .bind(("127.0.0.1", 9001))?
    .run()
    .await
}
