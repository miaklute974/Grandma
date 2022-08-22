use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Item {
  name: String,
  description: String,
  price: i32,
  completed: bool,
}

fn main() -> Result<()> {
  let conn = Connection::open_in_memory()?;

  conn.execute(
    "CREATE TABLE item (
      name TEXT NOT NULL,
      description TEXT,
      price INTEGER,
      completed BOOL
    )",
    (),
  )?;

  let first = Item {
    name: "Shirt".to_string(),
    description: "This is a shirt".to_string(),
    price: 20,
    completed: true
  };
  conn.execute(
    "INSERT INTO item (name, description, price, completed) VALUES (?1, ?2, ?3, ?4)",
    (&first.name, &first.description, &first.price, &first.completed),
  )?;

  let mut stmt = conn.prepare("SELECT name, description, price, completed FROM item")?;
  let item_iter = stmt.query_map([], |row| {
    Ok(Item {
      name: row.get(0)?,
      description: row.get(1)?,
      price: row.get(2)?,
      completed: row.get(3)?,
    })
  })?;

  for item in item_iter {
    println!("Found item {:?}", item.unwrap())
  }
  Ok(())
}