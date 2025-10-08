use crate::model::Model;
use rusqlite::{params_from_iter, Connection, Result as SqliteResult, Row};
use serde_json;
use std::path::Path;

/// Main database connection wrapper
pub struct Database {
    pub(crate) conn: Connection,
}

impl Database {
    /// Connect to or create a SQLite database file
    pub fn connect<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    /// Connect to an in-memory database (useful for testing)
    pub fn connect_in_memory() -> SqliteResult<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Database { conn })
    }

    /// Create a table for the given model
    pub fn create_table<T: Model>(&self) -> SqliteResult<()> {
        let table_name = T::table_name();
        let fields = T::fields();
        
        // Build CREATE TABLE statement
        // For simplicity, we'll use TEXT for most fields and INTEGER for id
        let mut field_definitions = Vec::new();
        for field in fields {
            if *field == T::primary_key() {
                field_definitions.push(format!("{} INTEGER PRIMARY KEY", field));
            } else {
                field_definitions.push(format!("{} TEXT", field));
            }
        }
        
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name,
            field_definitions.join(", ")
        );
        
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    /// Insert a model instance into the database
    pub fn insert<T: Model>(&self, model: &T) -> SqliteResult<i64> {
        let table_name = T::table_name();
        let fields = T::fields();
        
        // Serialize model to JSON to extract field values
        let json_value = serde_json::to_value(model)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        let json_obj = json_value.as_object()
            .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
        
        // Build field names and placeholders
        let field_names: Vec<&str> = fields.iter().copied().collect();
        let placeholders: Vec<String> = (0..fields.len()).map(|_| "?".to_string()).collect();
        
        // Extract values in the correct order
        let mut values: Vec<String> = Vec::new();
        for field in fields {
            let value = json_obj.get(*field)
                .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
            
            // Convert JSON value to string representation
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "NULL".to_string(),
                _ => serde_json::to_string(value)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
            };
            values.push(value_str);
        }
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            field_names.join(", "),
            placeholders.join(", ")
        );
        
        self.conn.execute(&sql, params_from_iter(values.iter()))?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Select all rows from a model's table
    pub fn select_all<T: Model>(&self) -> SqliteResult<Vec<T>> {
        let table_name = T::table_name();
        let fields = T::fields();
        
        let sql = format!(
            "SELECT {} FROM {}",
            fields.join(", "),
            table_name
        );
        
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            self.row_to_model::<T>(row, fields)
        })?;
        
        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }
        
        Ok(results)
    }

    /// Find a single row by primary key
    pub fn find_by_id<T: Model>(&self, id: i64) -> SqliteResult<Option<T>> {
        let table_name = T::table_name();
        let fields = T::fields();
        let pk = T::primary_key();
        
        let sql = format!(
            "SELECT {} FROM {} WHERE {} = ?",
            fields.join(", "),
            table_name,
            pk
        );
        
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query_map([id], |row| {
            self.row_to_model::<T>(row, fields)
        })?;
        
        if let Some(row_result) = rows.next() {
            Ok(Some(row_result?))
        } else {
            Ok(None)
        }
    }

    /// Delete a row by primary key
    pub fn delete<T: Model>(&self, id: i64) -> SqliteResult<usize> {
        let table_name = T::table_name();
        let pk = T::primary_key();
        
        let sql = format!(
            "DELETE FROM {} WHERE {} = ?",
            table_name,
            pk
        );
        
        self.conn.execute(&sql, [id])
    }

    /// Update a model instance in the database
    pub fn update<T: Model>(&self, model: &T) -> SqliteResult<usize> {
        let table_name = T::table_name();
        let fields = T::fields();
        let pk = T::primary_key();
        
        // Serialize model to JSON
        let json_value = serde_json::to_value(model)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        let json_obj = json_value.as_object()
            .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
        
        // Get primary key value
        let pk_value = json_obj.get(pk)
            .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
        let pk_str = match pk_value {
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => s.clone(),
            _ => return Err(rusqlite::Error::InvalidQuery),
        };
        
        // Build SET clause (excluding primary key)
        let mut set_clauses = Vec::new();
        let mut values: Vec<String> = Vec::new();
        
        for field in fields {
            if *field == pk {
                continue; // Skip primary key in UPDATE SET
            }
            
            set_clauses.push(format!("{} = ?", field));
            
            let value = json_obj.get(*field)
                .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
            
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "NULL".to_string(),
                _ => serde_json::to_string(value)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
            };
            values.push(value_str);
        }
        
        // Add primary key value for WHERE clause
        values.push(pk_str);
        
        let sql = format!(
            "UPDATE {} SET {} WHERE {} = ?",
            table_name,
            set_clauses.join(", "),
            pk
        );
        
        self.conn.execute(&sql, params_from_iter(values.iter()))
    }

    /// Helper to convert a Row to a Model instance
    fn row_to_model<T: Model>(&self, row: &Row, fields: &[&str]) -> SqliteResult<T> {
        let mut json_map = serde_json::Map::new();
        
        for (idx, field) in fields.iter().enumerate() {
            // Try to get the value as different types
            let value: serde_json::Value = if let Ok(v) = row.get::<_, i64>(idx) {
                serde_json::Value::Number(v.into())
            } else if let Ok(v) = row.get::<_, String>(idx) {
                serde_json::Value::String(v)
            } else if let Ok(v) = row.get::<_, f64>(idx) {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(v).unwrap_or_else(|| 0.into())
                )
            } else {
                serde_json::Value::Null
            };
            
            json_map.insert(field.to_string(), value);
        }
        
        let json_value = serde_json::Value::Object(json_map);
        serde_json::from_value(json_value)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e)
            ))
    }

    /// Drop a table (useful for testing)
    pub fn drop_table<T: Model>(&self) -> SqliteResult<()> {
        let table_name = T::table_name();
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    /// Create a query builder for this database
    pub fn query<T: Model>(&self) -> crate::query::QueryBuilder<'_> {
        crate::query::QueryBuilder::new::<T>(&self.conn)
    }
}

