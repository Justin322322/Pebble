use crate::model::Model;
use rusqlite::{Connection, Result as SqliteResult, Row, params_from_iter};
use serde_json;

/// Simple query builder for SELECT statements
pub struct QueryBuilder<'a> {
    conn: &'a Connection,
    table_name: String,
    fields: Vec<String>,
    where_clauses: Vec<String>,
    where_values: Vec<String>,
    order_by: Option<String>,
    limit: Option<usize>,
}

impl<'a> QueryBuilder<'a> {
    /// Create a new query builder
    pub fn new<T: Model>(conn: &'a Connection) -> Self {
        let table_name = T::table_name().to_string();
        let fields: Vec<String> = T::fields().iter().map(|s| s.to_string()).collect();
        
        QueryBuilder {
            conn,
            table_name,
            fields,
            where_clauses: Vec::new(),
            where_values: Vec::new(),
            order_by: None,
            limit: None,
        }
    }

    /// Add a WHERE clause
    pub fn where_eq(mut self, field: &str, value: impl ToString) -> Self {
        self.where_clauses.push(format!("{} = ?", field));
        self.where_values.push(value.to_string());
        self
    }

    /// Add a WHERE LIKE clause
    pub fn where_like(mut self, field: &str, pattern: impl ToString) -> Self {
        self.where_clauses.push(format!("{} LIKE ?", field));
        self.where_values.push(pattern.to_string());
        self
    }

    /// Add a WHERE > clause
    pub fn where_gt(mut self, field: &str, value: impl ToString) -> Self {
        self.where_clauses.push(format!("{} > ?", field));
        self.where_values.push(value.to_string());
        self
    }

    /// Add a WHERE < clause
    pub fn where_lt(mut self, field: &str, value: impl ToString) -> Self {
        self.where_clauses.push(format!("{} < ?", field));
        self.where_values.push(value.to_string());
        self
    }

    /// Add ORDER BY clause
    pub fn order_by(mut self, field: &str, ascending: bool) -> Self {
        let direction = if ascending { "ASC" } else { "DESC" };
        self.order_by = Some(format!("{} {}", field, direction));
        self
    }

    /// Add LIMIT clause
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Execute the query and return results
    pub fn fetch<T: Model>(self) -> SqliteResult<Vec<T>> {
        let mut sql = format!(
            "SELECT {} FROM {}",
            self.fields.join(", "),
            self.table_name
        );

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        if let Some(order) = self.order_by {
            sql.push_str(" ORDER BY ");
            sql.push_str(&order);
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        let mut stmt = self.conn.prepare(&sql)?;
        let fields_refs: Vec<&str> = self.fields.iter().map(|s| s.as_str()).collect();
        
        let rows = stmt.query_map(params_from_iter(self.where_values.iter()), |row| {
            row_to_model::<T>(row, &fields_refs)
        })?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }

        Ok(results)
    }

    /// Execute the query and return the first result
    pub fn fetch_one<T: Model>(self) -> SqliteResult<Option<T>> {
        let results = self.limit(1).fetch::<T>()?;
        Ok(results.into_iter().next())
    }
}

/// Helper function to convert a Row to a Model instance
fn row_to_model<T: Model>(row: &Row, fields: &[&str]) -> SqliteResult<T> {
    let mut json_map = serde_json::Map::new();
    
    for (idx, field) in fields.iter().enumerate() {
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

