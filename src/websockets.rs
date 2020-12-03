use warp::ws::{WebSocket, Ws};
use warp::Filter;

pub const WEBSOCKET_PATH: &str = "websocket";

/// Create a websocket server.
pub fn new() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let ws = warp::path(WEBSOCKET_PATH)
        .and(warp::ws())
        .map(|ws: Ws| ws.on_upgrade(move |socket| handler(socket)))
        .boxed();
    ws
}

/// Handle a websocket connection.
async fn handler(_ws: WebSocket) {
    println!("Websocket connection");
    // We do nothing with incoming messages for now.
}
