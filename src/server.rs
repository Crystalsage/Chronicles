use hyper::{Body, Response, Request, Server};
use std::convert::Infallible;

pub async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello".into()))
}
