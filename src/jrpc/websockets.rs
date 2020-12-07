use warp::ws::{WebSocket, Ws as WarpWs};
use warp::Filter;

/// Websocket endpoint.
pub struct Ws {}

impl Ws {
    /// Create a websocket server.
    pub fn new() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
        warp::ws()
            .map(|ws: WarpWs| ws.on_upgrade(move |socket| handler(socket)))
            .boxed()
    }
}

/// Handle a websocket connection.
async fn handler(_ws: WebSocket) {
    println!("Websocket connection");
    // We do nothing with incoming messages for now.
}
