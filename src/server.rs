use warp::Filter;
use crate::Message;

pub async fn run() {
    let base = warp::path::end();
    let rng = rand::thread_rng();

    let message = warp::post()
        .and(warp::path("getmsg"))
        .and(warp::path::param())
        .and(warp::body::json())
        .map(|content, mut msg: Message| {
            let msg = Message  {
                timestamp: 69420,
                content: content,
            };

            warp::reply::json(&msg)
        });

    let routes = warp::get().and(
        base
        .or(message));

    println!("Running server!");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
