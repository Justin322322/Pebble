use serde::{Deserialize, Serialize};

/// Core trait that all models must implement to map to database tables
pub trait Model: Sized + Serialize + for<'de> Deserialize<'de> {
    /// Returns the name of the database table
    fn table_name() -> &'static str;
    
    /// Returns the field names for the model
    fn fields() -> &'static [&'static str];
    
    /// Returns the primary key field name (defaults to "id")
    fn primary_key() -> &'static str {
        "id"
    }
}

