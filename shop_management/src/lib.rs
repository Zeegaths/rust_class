#[derive(Debug, PartialEq)]
struct Item {
    name: String,
    quantity: u32,
}

impl Item {
    fn new(name: &str, quantity: u32) -> Item {
        Item {
            name: name.to_string(),
            quantity,
        }
    }
}

struct Shop {
    items: Vec<Item>,
}

impl Shop {
    fn new() -> Shop {
        Shop { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn display_items(&self) -> String {
        let mut output = String::new();
        for item in &self.items {
            output.push_str(&format!("Item: {}, Quantity: {}\n", item.name, item.quantity));
        }
        output
    }
}

trait Summarize {
    fn summary(&self) -> String;
}

impl Summarize for Shop {
    fn summary(&self) -> String {
        let total_items = self.items.len();
        let total_quantity: u32 = self.items.iter().map(|item| item.quantity).sum();
        format!(
            "Total different items in stock: {}\nTotal quantity of all items: {}\n",
            total_items, total_quantity
        )
    }
}

fn main() {
    let mut shop = Shop::new();

    shop.add_item(Item::new("Apples", 30));
    shop.add_item(Item::new("Bananas", 45));
    shop.add_item(Item::new("Oranges", 25));

    println!("Items in stock:");
    print!("{}", shop.display_items());

    println!("\nSummary:");
    print!("{}", shop.summary());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_items() {
        let mut shop = Shop::new();
        shop.add_item(Item::new("Apples", 30));
        shop.add_item(Item::new("Bananas", 45));
        let output = shop.display_items();
        let expected_output = "Item: Apples, Quantity: 30\nItem: Bananas, Quantity: 45\n";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_summary() {
        let mut shop = Shop::new();
        shop.add_item(Item::new("Apples", 30));
        shop.add_item(Item::new("Bananas", 45));
        let output = shop.summary();
        let expected_output = "Total different items in stock: 2\nTotal quantity of all items: 75\n";
        assert_eq!(output, expected_output);
    }
}
