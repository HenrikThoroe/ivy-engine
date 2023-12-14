//! Implementation for the UCI protocol.
//!
//! Provides types and functions to parse and build UCI commands and messages.
//! Does not have an opinion about how to handle commands or messages.

mod cmd;
mod msg;

pub use cmd::*;
pub use msg::*;
