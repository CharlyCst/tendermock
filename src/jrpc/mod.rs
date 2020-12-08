//! # JsonRPC
//!
//! This module contains the the utilities needed do build the JsonRPC api, both HTTP and
//! WebSockets, which can then directly served by `warp`.
//!
//! `warp` is a HTTP framework which is built around filters, this module is used to build filters
//! that mimics the Tendermint API.
mod api;
mod utils;
mod websockets;

pub use api::Jrpc;
pub use websockets::Ws;
