#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;

use http::{Request, Response};

mod server;

mod schema {
    use rand::Rng;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Platform {
        IRC,
        Discord,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Post {
        pub id: usize,
        pub platform: Platform,
        pub messages: Vec<Message>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub timestamp: usize,
        pub content: String,
    }

    impl Platform {
        // Returns a random platform
        pub fn choose_platform() -> Self {
            let mut rng = rand::thread_rng();

            match rng.gen_range(0..2) {
                0 => Platform::IRC , 
                1 => Platform::Discord,
                _ => panic!("RNG failed to get Platform!"),
            }
        }
    }

    impl Message {
        // count: number of messages in a post
        pub fn new(count: usize) -> Vec<Self> {
            let mut messages: Vec<Message> = Vec::new();

            for message in 0..count {
                messages.push(Message {
                    timestamp: rand::thread_rng().gen_range(10000..99999),
                    content: String::from(format!("I am message number {}!", message+1)),
                });
            }

            return messages;
        }

        pub async fn from_url(url: String) -> Self {
            let mut message_string = String::new();

            Message {
                timestamp: rand::thread_rng().gen_range(10000..99999),
                content: message_body,
            }
        }
    }
                    
    impl Post {
        pub fn new(message_count: Option<usize>) -> Self {
            Post {
                // TODO: Change hardcoded values
                id: 1,
                messages: Message::new(message_count.unwrap_or(0)),
                platform: Platform::choose_platform(),
            }
        }
    }
}

type Message = schema::Message;
type Platform = schema::Platform;
type Post = schema::Post;

#[actix_web::main]
async fn main() {
    // let server = server::run().await;
    let message = Message::from_url("https://google.com".to_string());
    dbg!(message);
}
