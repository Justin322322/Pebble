use pebble::{Database, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Hero {
    id: i32,
    name: String,
    attribute: String,
}

impl Model for Hero {
    fn table_name() -> &'static str {
        "heroes"
    }

    fn fields() -> &'static [&'static str] {
        &["id", "name", "attribute"]
    }
}

fn main() -> pebble::Result<()> {
    println!("Pebble - Dota 2 Heroes Example\n");

    // Connect to database
    let db = Database::connect("dota.db")?;
    println!("Connected to database");

    // Create table
    db.create_table::<Hero>()?;
    println!("Created heroes table");

    // Insert heroes with their primary attributes
    let invoker = Hero {
        id: 1,
        name: "Invoker".into(),
        attribute: "Intelligence".into(),
    };

    let juggernaut = Hero {
        id: 2,
        name: "Juggernaut".into(),
        attribute: "Agility".into(),
    };

    let axe = Hero {
        id: 3,
        name: "Axe".into(),
        attribute: "Strength".into(),
    };

    db.insert(&invoker)?;
    db.insert(&juggernaut)?;
    db.insert(&axe)?;
    println!("Inserted 3 heroes");

    // Select all heroes
    let heroes = db.select_all::<Hero>()?;
    println!("\nAll heroes:");
    for hero in &heroes {
        println!("  - {} ({})", hero.name, hero.attribute);
    }

    // Find by ID
    if let Some(hero) = db.find_by_id::<Hero>(1)? {
        println!("\nFound hero by ID 1: {} ({})", hero.name, hero.attribute);
    }

    // Update hero
    let updated_invoker = Hero {
        id: 1,
        name: "Invoker".into(),
        attribute: "Intelligence".into(),
    };
    db.update(&updated_invoker)?;
    println!("\nUpdated Invoker");

    // Verify update
    if let Some(hero) = db.find_by_id::<Hero>(1)? {
        println!("   Hero: {} ({})", hero.name, hero.attribute);
    }

    // Delete hero
    db.delete::<Hero>(2)?;
    println!("\nDeleted Juggernaut (ID 2)");

    // Show remaining heroes
    let remaining_heroes = db.select_all::<Hero>()?;
    println!("\nRemaining heroes: {}", remaining_heroes.len());
    for hero in &remaining_heroes {
        println!("  - {} ({})", hero.name, hero.attribute);
    }

    println!("\nExample completed successfully!");

    Ok(())
}
