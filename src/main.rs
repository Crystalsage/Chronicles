#![allow(dead_code)]
#![allow(unused_variables)]

mod schema {
    use rand::Rng;

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

    #[derive(Debug)]
    pub struct Message {
        timestamp: usize,
        content: String,
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
                platform: Platform::IRC,
            }
        }
    }
}

type Message = schema::Message;
type Platform = schema::Platform;
type Post = schema::Post;

fn main() {
    let post: Post = Post::new(Some(8));
    dbg!(post);
}
