# Pebble

A lightweight Rust ORM (Object-Relational Mapper) built on top of rusqlite.

Pebble provides a minimal, type-safe abstraction layer for mapping Rust structs to SQLite database tables with CRUD operations, without the complexity of a full ORM like Diesel or SeaORM.

## Project Goals

- Provide a simple API for defining models and performing CRUD operations
- Use Rust traits and generics to ensure compile-time type safety
- Avoid macros - use derive traits and manual implementations
- Serve as an educational ORM prototype, not a full ORM replacement

## Features

- **Model Derivation** - Define a struct and implement the Model trait to map it to a database table
- **CRUD Operations** - Basic Create, Read, Update, Delete functions
- **Query Builder** - Small builder for simple SELECT queries with filtering, ordering, and limiting
- **Type Conversion** - Safe conversion between Rust types and SQLite columns
- **SQL Injection Protection** - Parameterized queries protect against SQL injection
- **Unit Tests** - Comprehensive test suite for model persistence and queries

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pebble = "0.1.0"
rusqlite = "0.31"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Quick Start

```rust
use pebble::{Model, Database};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

impl Model for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn fields() -> &'static [&'static str] {
        &["id", "name", "email"]
    }
}

fn main() -> pebble::Result<()> {
    // Connect to database
    let db = Database::connect("pebble.db")?;

    // Create table
    db.create_table::<User>()?;

    // Insert a user
    let user = User {
        id: 1,
        name: "Alice".into(),
        email: "alice@example.com".into(),
    };
    db.insert(&user)?;

    // Retrieve all users
    let users = db.select_all::<User>()?;
    println!("{:?}", users);

    Ok(())
}
```

## API Documentation

### Database Operations

#### Connect to Database

```rust
// File-based database
let db = Database::connect("myapp.db")?;

// In-memory database (useful for testing)
let db = Database::connect_in_memory()?;
```

#### Create Table

```rust
db.create_table::<User>()?;
```

#### Insert

```rust
let user = User { id: 1, name: "Alice".into(), email: "alice@example.com".into() };
let row_id = db.insert(&user)?;
```

#### Select All

```rust
let users = db.select_all::<User>()?;
```

#### Find by ID

```rust
if let Some(user) = db.find_by_id::<User>(1)? {
    println!("Found: {:?}", user);
}
```

#### Update

```rust
let updated_user = User { id: 1, name: "Alice Smith".into(), email: "alice.smith@example.com".into() };
db.update(&updated_user)?;
```

#### Delete

```rust
db.delete::<User>(1)?;
```

#### Drop Table

```rust
db.drop_table::<User>()?;
```

### Query Builder

For more complex queries, use the QueryBuilder:

```rust
use pebble::QueryBuilder;

// Filter by field
let results = QueryBuilder::new::<User>(&db.conn)
    .where_eq("name", "Alice")
    .fetch::<User>()?;

// Order results
let results = QueryBuilder::new::<User>(&db.conn)
    .order_by("name", true)  // true = ascending, false = descending
    .fetch::<User>()?;

// Limit results
let results = QueryBuilder::new::<User>(&db.conn)
    .limit(10)
    .fetch::<User>()?;

// Combine filters
let results = QueryBuilder::new::<Product>(&db.conn)
    .where_eq("category", "Electronics")
    .where_lt("price", "100")
    .order_by("price", true)
    .limit(5)
    .fetch::<Product>()?;

// Fetch single result
let result = QueryBuilder::new::<User>(&db.conn)
    .where_eq("email", "alice@example.com")
    .fetch_one::<User>()?;
```

#### Query Builder Methods

- `.where_eq(field, value)` - WHERE field = value
- `.where_like(field, pattern)` - WHERE field LIKE pattern
- `.where_gt(field, value)` - WHERE field > value
- `.where_lt(field, value)` - WHERE field < value
- `.order_by(field, ascending)` - ORDER BY field ASC/DESC
- `.limit(n)` - LIMIT n
- `.fetch::<T>()` - Execute and return Vec<T>
- `.fetch_one::<T>()` - Execute and return Option<T>

## Defining Models

To use Pebble, implement the Model trait for your structs:

```rust
use pebble::Model;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
    author_id: i32,
}

impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn fields() -> &'static [&'static str] {
        &["id", "title", "content", "author_id"]
    }
    
    // Optional: override primary key (defaults to "id")
    fn primary_key() -> &'static str {
        "id"
    }
}
```

**Requirements:**
- Structs must derive `Serialize` and `Deserialize` from serde
- Implement `table_name()` to specify the database table name
- Implement `fields()` to list all field names in order
- Optionally override `primary_key()` if not using "id"

## Building and Testing

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Run Examples

```bash
cargo run --example basic_usage
cargo run --example query_builder
```

## Project Structure

```
pebble/
├── Cargo.toml
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── src/
│   ├── lib.rs          # Public API exports
│   ├── db.rs           # Database struct and CRUD operations
│   ├── model.rs        # Model trait definition
│   ├── query.rs        # Query builder implementation
│   └── tests.rs        # Unit tests
└── examples/
    ├── basic_usage.rs  # Basic CRUD example
    └── query_builder.rs # Query builder example
```

## Dependencies

- **rusqlite** (0.31) - SQLite wrapper for Rust
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON support for serde

## Security

Pebble uses parameterized queries to protect against SQL injection attacks. All user-provided values are safely bound to query parameters rather than concatenated into SQL strings.

## Limitations

This is an educational ORM prototype. It has several limitations:

- **Schema flexibility**: All non-primary-key fields are stored as TEXT
- **Type support**: Limited to basic types (integers, strings, floats)
- **Relationships**: No built-in support for foreign keys or joins
- **Migrations**: No automated schema migration tools
- **Performance**: Not optimized for high-performance scenarios
- **Async**: No async/await support

## Future Enhancements

Potential improvements for future versions:

- Support for foreign keys and relationships
- Simple query macros (`find_by!`, etc.)
- Async support (via tokio + sqlx)
- CLI migration tool (`pebble migrate`)
- Better type mapping (integers, booleans, dates)
- Connection pooling
- Transactions support
- Derive macros for Model trait

## License

MIT

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Acknowledgments

Built with:
- [rusqlite](https://github.com/rusqlite/rusqlite) - Ergonomic SQLite bindings for Rust
- [serde](https://serde.rs/) - Serialization framework for Rust
