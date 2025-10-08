use pebble::{Database, Model, QueryBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: i32,
    name: String,
    category: String,
    cost: i32,
}

impl Model for Item {
    fn table_name() -> &'static str {
        "items"
    }

    fn fields() -> &'static [&'static str] {
        &["id", "name", "category", "cost"]
    }
}

fn main() -> pebble::Result<()> {
    println!("Pebble - Dota 2 Items Query Example\n");

    // Connect and setup
    let db = Database::connect("dota_items.db")?;
    db.create_table::<Item>()?;
    println!("Database ready");

    // Insert items with accurate costs based on Dota 2
    let items = vec![
        // Basic items
        Item {
            id: 1,
            name: "Iron Branch".into(),
            category: "Basic".into(),
            cost: 50,
        },
        Item {
            id: 2,
            name: "Magic Stick".into(),
            category: "Basic".into(),
            cost: 200,
        },
        Item {
            id: 3,
            name: "Boots of Speed".into(),
            category: "Basic".into(),
            cost: 500,
        },
        // Support items
        Item {
            id: 4,
            name: "Observer Ward".into(),
            category: "Support".into(),
            cost: 0,
        },
        Item {
            id: 5,
            name: "Sentry Ward".into(),
            category: "Support".into(),
            cost: 75,
        },
        // Upgraded items
        Item {
            id: 6,
            name: "Power Treads".into(),
            category: "Upgrades".into(),
            cost: 1400,
        },
        Item {
            id: 7,
            name: "Blink Dagger".into(),
            category: "Upgrades".into(),
            cost: 2250,
        },
        // High tier items
        Item {
            id: 8,
            name: "Black King Bar".into(),
            category: "Armor".into(),
            cost: 4050,
        },
        Item {
            id: 9,
            name: "Daedalus".into(),
            category: "Weapons".into(),
            cost: 5150,
        },
        Item {
            id: 10,
            name: "Divine Rapier".into(),
            category: "Weapons".into(),
            cost: 5600,
        },
        Item {
            id: 11,
            name: "Aghanims Scepter".into(),
            category: "Caster".into(),
            cost: 4200,
        },
    ];

    for item in &items {
        db.insert(item)?;
    }
    println!("Inserted {} items\n", items.len());

    // Example 1: Filter by category - Basic items
    println!("Basic starting items:");
    let basic_items = QueryBuilder::new::<Item>(&db.conn)
        .where_eq("category", "Basic")
        .fetch::<Item>()?;
    
    for item in &basic_items {
        println!("  - {} ({} gold)", item.name, item.cost);
    }

    // Example 2: Order by cost (most expensive first)
    println!("\nMost expensive items:");
    let by_price = QueryBuilder::new::<Item>(&db.conn)
        .order_by("cost", false)
        .limit(5)
        .fetch::<Item>()?;
    
    for item in &by_price {
        println!("  - {} - {} gold ({})", item.name, item.cost, item.category);
    }

    // Example 3: Affordable early game items under 1000 gold
    println!("\nAffordable early game items (under 1000 gold):");
    let affordable = QueryBuilder::new::<Item>(&db.conn)
        .where_lt("cost", "1000")
        .order_by("cost", true)
        .fetch::<Item>()?;
    
    for item in &affordable {
        println!("  - {} - {} gold", item.name, item.cost);
    }

    // Example 4: Find specific item
    println!("\nFind Blink Dagger:");
    let result = QueryBuilder::new::<Item>(&db.conn)
        .where_eq("name", "Blink Dagger")
        .fetch_one::<Item>()?;
    
    if let Some(item) = result {
        println!("  Found: {} - {} gold ({})", item.name, item.cost, item.category);
    }

    // Example 5: High tier items over 4000 gold
    println!("\nHigh tier items (over 4000 gold):");
    let expensive = QueryBuilder::new::<Item>(&db.conn)
        .where_gt("cost", "4000")
        .order_by("cost", true)
        .fetch::<Item>()?;
    
    for item in &expensive {
        println!("  - {} - {} gold ({})", item.name, item.cost, item.category);
    }

    // Example 6: Support items
    println!("\nSupport items:");
    let support_items = QueryBuilder::new::<Item>(&db.conn)
        .where_eq("category", "Support")
        .fetch::<Item>()?;
    
    for item in &support_items {
        if item.cost == 0 {
            println!("  - {} (Free)", item.name);
        } else {
            println!("  - {} - {} gold", item.name, item.cost);
        }
    }

    // Example 7: Weapon category items
    println!("\nWeapon items:");
    let weapons = QueryBuilder::new::<Item>(&db.conn)
        .where_eq("category", "Weapons")
        .order_by("cost", false)
        .fetch::<Item>()?;
    
    for item in &weapons {
        println!("  - {} - {} gold", item.name, item.cost);
    }

    println!("\nQuery examples completed!");

    Ok(())
}
