//! # A message logger to prettify outputs
//!
//! This module defines a `log!` macro that takes care of formatting logs output. The macro takes
//! the identity of the logger (see `Log` enum) and behaves as `format!` for the remaining arguments.
//!
//! ```
//! log!(Log::JRPC, "query: {}", "/example")
//! ```

use chrono::Utc;
use colored::*;

/// The list of entitites that can emit logs.
pub enum Log {
    JRPC,
    GRPC,
    Chain,
    Websocket,
}

impl Log {
    pub fn as_str(self) -> ColoredString {
        match self {
            Log::Websocket => "[Websocket]".cyan(),
            Log::JRPC => "[JsonRPC]".yellow(),
            Log::Chain => "[Chain]".magenta(),
            Log::GRPC => "[gRPC]".green(),
        }
    }
}

/// Return a formatted string of the curent time.
pub fn now() -> ColoredString {
    let now = Utc::now().format("%H:%M:%S");
    now.to_string().blue()
}

#[macro_escape]
macro_rules! log {
    ($logger:expr, $str:expr, $($params:expr),*) => {
        let fmt_str = format!($str, $($params,)*);
        println!("{} {:>11} {}",crate::logger::now(),$logger.as_str(), fmt_str);
    };
    ($logger:expr, $str:expr) => {
        println!("{} {:>11} {}", crate::logger::now(), $logger.as_str(), $str);
    };
}
