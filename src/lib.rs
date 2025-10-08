//! # Pebble
//!
//! A lightweight Rust ORM built on top of rusqlite.
//!
//! ## Example
//!
//! ```rust
//! use pebble::{Model, Database};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {
//!     id: i32,
//!     name: String,
//!     email: String,
//! }
//!
//! impl Model for User {
//!     fn table_name() -> &'static str {
//!         "users"
//!     }
//!
//!     fn fields() -> &'static [&'static str] {
//!         &["id", "name", "email"]
//!     }
//! }
//!
//! fn main() -> pebble::Result<()> {
//!     let db = Database::connect("pebble.db")?;
//!     db.create_table::<User>()?;
//!
//!     let user = User {
//!         id: 1,
//!         name: "Alice".into(),
//!         email: "alice@example.com".into(),
//!     };
//!
//!     db.insert(&user)?;
//!     let users = db.select_all::<User>()?;
//!
//!     println!("{:?}", users);
//!     Ok(())
//! }
//! ```

mod db;
mod model;
mod query;

#[cfg(test)]
mod tests;

// Re-export main types
pub use db::Database;
pub use model::Model;
pub use query::QueryBuilder;

// Re-export rusqlite Result type for convenience
pub use rusqlite::Result;

