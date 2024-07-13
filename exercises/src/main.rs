// use std::collections::HashMap;


// //two vectors
// //1st - 1-50
// //2nd - 50 - 100
// //hashmap - 1st vector as key, second vector as value, in decesnding order

// fn main() {
//     // Create the first vector with numbers 1-50
//     let vec1: Vec<i32> = (1..=50).collect();

//     // Create the second vector with numbers 51-100
//     let vec2: Vec<i32> = (51..=100).rev().collect();

//     // Create a HashMap to store the elements in descending order
//     let mut hashmap: HashMap<i32, i32> = HashMap::new();

//     // Insert elements into the HashMap in descending order
//     for (index, item) in vec1.iter().enumerate(){
//         let value = vec2.get(index).unwrap();
//         hashmap.insert(*item, *value);
//     }

//     // Print the HashMap to verify the result
//     for (key, value) in &hashmap {
//         println!("{}: {}", key, value);
//     }
// }


// using match instead of unwarap 
// use std::collections::HashMap;

// fn main() {
//     // Create the first vector with numbers 1-50
//     let vec1: Vec<i32> = (1..=50).collect();

//     // Create the second vector with numbers 51-100 in reverse order
//     let vec2: Vec<i32> = (51..=100).rev().collect();

//     // Create a HashMap to store the elements in descending order
//     let mut hashmap: HashMap<i32, i32> = HashMap::new();

//     // Insert elements into the HashMap in descending order
//     for (index, item) in vec1.iter().enumerate() {
//         match vec2.get(index) {
//             Some(&value) => {
//                 hashmap.insert(*item, value);
//             }
//             None => {
//                 eprintln!("No value found for index: {}", index);
//             }
//         }
//     }

//     // Print the HashMap to verify the result
//     for (key, value) in &hashmap {
//         println!("{}: {}", key, value);
//     }
// }



//shop management sstem storing items and quantity. using structs and traits,
// create solution and use trait to summarize the stock
use std::collections::HashMap;

// Define a struct for an item in the shop
#[derive(Debug)]
struct Item {
    name: String,
    quantity: u32,
}


// Define a trait for summarizing stock
trait StockSummary {
    fn summarize_stock(&self);
}

// Implement the StockSummary trait for Shop
impl StockSummary for Item {
    fn summarize_stock(&self) {       
    println!("Item: {}, Quantity: {}", self.name, self.quantity)    
}
}

fn main() {
    let vec:Vec<Item> = Vec::new();
    let item = Item{name:"Pen".to_string(), quantity: 10};
    item.summarize_stock();

}

