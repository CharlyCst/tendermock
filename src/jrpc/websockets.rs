//! The Tendermock JsonRPC Websocket API.
use warp::ws::{WebSocket, Ws as WarpWs};
use warp::Filter;

/// A struct that can be used to build the Websocket `warp` filter, see the `new` method.
pub struct Ws {}

impl Ws {
    /// Creates a `warp` filter that mimics the Tendermint Websocket API.
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
