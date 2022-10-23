#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;

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
        pub author: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct DiscordMessages {
        pub messages: Vec<String>
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
        pub fn new(discord_messages: Vec<&str>) -> Vec<Self> {
            let mut messages: Vec<Message> = Vec::new();

            for msg in discord_messages {
                messages.push(Message {
                    timestamp: 129302193,
                    content: msg.to_string(), 
                    author: "Bourbon".into() })
            }

            return messages;
        }

        pub fn random(count: u8) -> Vec<Self> {
            let mut messages: Vec<Message> = Vec::new();

            for i in 0..count {
                messages.push(
                    Message { 
                        timestamp: 182981923, 
                        content: format!("Hello Bourbon with id {}", i),
                        author: "Bourbon".to_string() });
            }

            return messages;
        }
    }
                    
    impl Post {
        pub fn new(messages: Vec<Message>) -> Self {
            Post {
                // TODO: Change hardcoded values
                id: 1,
                messages: messages,
                platform: Platform::Discord,
            }
        }
    }
}

type Message = schema::Message;
type Platform = schema::Platform;
type Post = schema::Post;
type DiscordMessages = schema::DiscordMessages;

#[actix_web::main]
async fn main() {
    let server = server::run().await;
}
