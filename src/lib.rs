//! The event vocabulary and state transition shared by Harness Nexuses.

mod generated {
    pub mod library;
}
mod transition;

pub use generated::library::*;
