//! Re-exports the upstream Discord API data model modules used by Project Phoenix.
//!
//! Keep networking and authentication in the supported integration layer; this module
//! only exposes serializable data structures for app-side parsing and state handling.

pub use discord_client_structs::{deserializer, parser, serializer, structs};
