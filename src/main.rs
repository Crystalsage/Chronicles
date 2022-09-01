#![allow(dead_code)]
#![allow(unused_variables)]


mod server {
    use std::convert::Infallible;
    use std::net::SocketAddr;
    use hyper::{Body, Request, Response, Server};
    use hyper::service::{make_service_fn, service_fn};

    async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new("Hello".into()))
    }

    #[tokio::main]
    async fn main() {
        let addr = SocketAddr::from(([127,0,0,1], 8001));
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(hello_world));
        });

        let server = Server::bind(&addr).serve(make_svc);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

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

fn main() {
    let post: Post = Post::new(Some(8));
    dbg!(post);
}
