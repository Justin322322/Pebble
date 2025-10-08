use crate::{Database, Model, QueryBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}

#[test]
fn test_create_table() {
    let db = Database::connect_in_memory().unwrap();
    let result = db.create_table::<User>();
    assert!(result.is_ok());
}

#[test]
fn test_model_trait() {
    assert_eq!(User::table_name(), "users");
    assert_eq!(User::fields(), &["id", "name", "email"]);
    assert_eq!(User::primary_key(), "id");
}

#[test]
fn test_insert_select() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let insert_result = db.insert(&user);
    assert!(insert_result.is_ok());

    let users = db.select_all::<User>().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[0].email, "alice@example.com");
}

#[test]
fn test_insert_multiple() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    let users_to_insert = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        },
    ];

    for user in &users_to_insert {
        db.insert(user).unwrap();
    }

    let users = db.select_all::<User>().unwrap();
    assert_eq!(users.len(), 3);
}

#[test]
fn test_find_by_id() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    let user = User {
        id: 42,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    db.insert(&user).unwrap();

    let found_user = db.find_by_id::<User>(42).unwrap();
    assert!(found_user.is_some());
    
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, 42);
    assert_eq!(found_user.name, "Alice");

    // Test non-existent ID
    let not_found = db.find_by_id::<User>(999).unwrap();
    assert!(not_found.is_none());
}

#[test]
fn test_delete() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    db.insert(&user).unwrap();

    let users_before = db.select_all::<User>().unwrap();
    assert_eq!(users_before.len(), 1);

    let delete_result = db.delete::<User>(1);
    assert!(delete_result.is_ok());
    assert_eq!(delete_result.unwrap(), 1);

    let users_after = db.select_all::<User>().unwrap();
    assert_eq!(users_after.len(), 0);

    // Delete non-existent record
    let delete_missing = db.delete::<User>(999).unwrap();
    assert_eq!(delete_missing, 0);
}

#[test]
fn test_update() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    db.insert(&user).unwrap();

    let updated_user = User {
        id: 1,
        name: "Alice Smith".to_string(),
        email: "alice.smith@example.com".to_string(),
    };

    let update_result = db.update(&updated_user);
    assert!(update_result.is_ok());
    assert_eq!(update_result.unwrap(), 1);

    let found = db.find_by_id::<User>(1).unwrap().unwrap();
    assert_eq!(found.name, "Alice Smith");
    assert_eq!(found.email, "alice.smith@example.com");
}

#[test]
fn test_query_builder_where_eq() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    db.insert(&User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }).unwrap();

    db.insert(&User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    }).unwrap();

    let query = QueryBuilder::new::<User>(&db.conn)
        .where_eq("name", "Alice");

    let results = query.fetch::<User>().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Alice");
}

#[test]
fn test_query_builder_limit() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    for i in 1..=5 {
        db.insert(&User {
            id: i,
            name: format!("User{}", i),
            email: format!("user{}@example.com", i),
        }).unwrap();
    }

    let query = QueryBuilder::new::<User>(&db.conn)
        .limit(3);

    let results = query.fetch::<User>().unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_query_builder_order_by() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    db.insert(&User {
        id: 1,
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
    }).unwrap();

    db.insert(&User {
        id: 2,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }).unwrap();

    db.insert(&User {
        id: 3,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    }).unwrap();

    let query = QueryBuilder::new::<User>(&db.conn)
        .order_by("name", true);

    let results = query.fetch::<User>().unwrap();
    assert_eq!(results[0].name, "Alice");
    assert_eq!(results[1].name, "Bob");
    assert_eq!(results[2].name, "Charlie");
}

#[test]
fn test_query_builder_fetch_one() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    db.insert(&User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }).unwrap();

    let query = QueryBuilder::new::<User>(&db.conn)
        .where_eq("id", 1);

    let result = query.fetch_one::<User>().unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Alice");
}

#[test]
fn test_sql_injection_protection() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    db.insert(&User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }).unwrap();

    // Attempt SQL injection through query builder
    let malicious_input = "Alice'; DROP TABLE users; --";
    let query = QueryBuilder::new::<User>(&db.conn)
        .where_eq("name", malicious_input);

    let results = query.fetch::<User>().unwrap();
    // Should return 0 results since no user has that exact name
    assert_eq!(results.len(), 0);

    // Verify table still exists
    let all_users = db.select_all::<User>().unwrap();
    assert_eq!(all_users.len(), 1);
}

#[test]
fn test_multiple_models() {
    let db = Database::connect_in_memory().unwrap();
    
    db.create_table::<User>().unwrap();
    db.create_table::<Post>().unwrap();

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    db.insert(&user).unwrap();

    let post = Post {
        id: 1,
        title: "My First Post".to_string(),
        content: "Hello, World!".to_string(),
        author_id: 1,
    };
    db.insert(&post).unwrap();

    let users = db.select_all::<User>().unwrap();
    let posts = db.select_all::<Post>().unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].author_id, users[0].id);
}

#[test]
fn test_drop_table() {
    let db = Database::connect_in_memory().unwrap();
    db.create_table::<User>().unwrap();

    db.insert(&User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }).unwrap();

    let drop_result = db.drop_table::<User>();
    assert!(drop_result.is_ok());

    // Recreate table - should be empty
    db.create_table::<User>().unwrap();
    let users = db.select_all::<User>().unwrap();
    assert_eq!(users.len(), 0);
}

