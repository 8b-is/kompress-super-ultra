//! HTTP and WebSocket request handlers for `kompress-super-ultra`.

pub mod kompress_handler;

pub use kompress_handler::{get_history, get_stats, handle_kompress, serve_webui, ws_handler};
