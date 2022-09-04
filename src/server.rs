use core::fmt;
use std::io::Error;
use std::sync::Mutex;

use actix_web::HttpResponse;
use actix_web::web::Data;
use actix_web::{get, web,  post, App, HttpServer, HttpRequest, error::PathError};
use redis::Commands;

use crate::{Message, Post, Platform};

use serde::Deserialize;

extern crate redis;


// Redis notes:
// Table 0 is for posts
// table 1 is for messages
//
mod signals {
    enum PostSignal {
        Success,
        Failure
    }

    pub struct PostError;
    struct MessageError;
}

type PostError = signals::PostError;
type JSONString = std::string::String;

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not create a post!")
    }
}

// Deserialize the String obtained from Redis 
// into a Post struct
fn string_to_post(post_string: &String) -> Option<Post> {
    serde_json::from_str(&post_string).ok()?
}


#[post("/create_post")]
async fn create_post(data: Data<Mutex<redis::Connection>>) -> Result<String, PathError> {
    let post: Post = Post {
        id: 2,
        platform: Platform::IRC,
        messages: get_messages(),
    };

    let post_json = serde_json::to_string(&post).unwrap();
    let mut con = data.lock().unwrap();
    con.set::<String, String, String>(post.id.to_string(), post_json)
        .expect("Redis SET failed for POST");

    Ok(String::from("0"))
}


// Returns a parsable JSON string 
#[get("/get_post_with_id/{post_id}")]
async fn get_post_with_id(data: Data<Mutex<redis::Connection>>, path: web::Path<String>) -> HttpResponse {
    let mut con = data.lock().unwrap();
    let post_id: String = path.into_inner();

    let post: JSONString = con.get(post_id).unwrap();

    HttpResponse::Ok().json(post)
}

#[get("/")]
async fn hello_there() -> Result<String, PathError> {
    let returnable = String::from("Hello there this is the actix server!");
    Ok(returnable)
}

fn get_messages() -> Vec<Message> {
    return Message::new(5);
}


//fn get_messages_new(message_urls: Vec<String>) -> Vec<Message> {
//    return Message::from_url(5);
//}

pub async fn run() -> std::io::Result<()>{
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    let con = Data::new(Mutex::new(con));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&con))
            .service(hello_there)
            .service(create_post)
            .service(get_post_with_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
