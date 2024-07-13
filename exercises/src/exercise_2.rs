//shop management sstem storing items and quantity. using structs and traits,
// create solution and use trait to summarize the stock
use std::collections::HashMap;

// Define a struct for an item in the shop
#[derive(Debug)]
struct Item {
    name: String,
    quantity: u32,
}

// Define a struct for the shop
#[derive(Debug)]
struct Shop {
    items: HashMap<String, Item>,
}

// Define a trait for summarizing stock
trait StockSummary {
    fn summarize_stock(&self);
}

// Implement the StockSummary trait for Shop
impl StockSummary for Shop {
    fn summarize_stock(&self) {
        println!("Stock Summary:");
        for (key, item) in &self.items {
            println!("Item: {}, Quantity: {}", key, item.quantity);
        }
    }
}

// Implement methods for Shop
impl Shop {
    // Create a new shop
    fn new() -> Shop {
        Shop {
            items: HashMap::new(),
        }
    }

    // Add an item to the shop
    fn add_item(&mut self, name: String, quantity: u32) {
        let item = Item { name: name.clone(), quantity };
        self.items.insert(name, item);
    }

    // Update the quantity of an existing item
    fn update_quantity(&mut self, name: &str, quantity: u32) {
        if let Some(item) = self.items.get_mut(name) {
            item.quantity = quantity;
        } else {
            println!("Item not found: {}", name);
        }
    }
}

fn main() {
    // Create a new shop
    let mut shop = Shop::new();

    // Add items to the shop
    shop.add_item("Apples".to_string(), 30);
    shop.add_item("Bananas".to_string(), 20);
    shop.add_item("Oranges".to_string(), 40);

    // Update the quantity of an item
    shop.update_quantity("Bananas", 25);

    // Summarize the stock
    shop.summarize_stock();
}


