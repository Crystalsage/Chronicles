#![allow(dead_code)]
#![allow(unused_variables)]

mod server;

mod schema {
    use rand::Rng;
    use serde::Serialize;

    #[derive(Debug)]
    pub enum Platform {
        IRC,
        Discord,
    }

    #[derive(Debug)]
    pub struct Post {
        platform: Platform,
        messages: Vec<Message>,
    }

    #[derive(Serialize, Debug)]
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
    }
                    
    impl Post {
        pub fn new(message_count: Option<usize>) -> Self {
            Post {
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
    let server = server::run().await;
}
