#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec;

use serde_json::json;

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
        // count: number of messages in a post
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

        pub async fn from_url() -> Self {
            let url: String = "https://gist.githubusercontent.com/Crystalsage/73fccaac6ec9a55377fcd43bd37aac12/raw/4b1022b8f1cfdb07ecd67de8f7acc1b5f219f035/messages.json".to_string();

            reqwest::get(url)
                .await.unwrap()
                .json::<Message>()
                .await.unwrap()
        }
    }
                    
    impl Post {
        pub fn new(message_count: Option<usize>) -> Self {
            Post {
                // TODO: Change hardcoded values
                id: 1,
                messages: Message::new(vec!["Hello", "There"]),
                platform: Platform::choose_platform(),
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
