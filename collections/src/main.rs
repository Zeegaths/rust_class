// fn main() {
    // let  mut x = vec![1, 2, 3, 4, 5];

    // #[doc = "This is the first solution to looping through the `vector`"]
    // for i in x {
    //     println!("Looping through {i}"x[i]);
    // }

    // let mut i = 0;
    // while i < x.len() {
    // println!("The number is {:?}", x[i]);
    // i += 1
    // }

    // let mut x = Vec::new();
    // x. push(1);
    // x. push(5);
    // x. push(3);

    // for i in x.iter_mut() {
    //     *i += 10
    // }

    // for i in x {
    //     let mut y = i + 1;

    //     println!("Looping through {i} and {y}");
    // }
// } 


// let date_time_string = "2024-05-23T12:34:56";
// let date: Vec<&str> = date_time_string.split('T'). collect();
// println!("The time is{:?}", date[1]);

// }

//Hashmaps

use std::collections::HashMap;
fn main () {
    let mut maps: HashMap<String, i32>= HashMap::new();
    maps.insert("Hello".to_string(), 22);
    maps.insert("world".to_string(), 30);
    maps.insert("Nigeria".to_string(), 34);

    println!("{:?}", maps);

    maps.insert("Hello".to_string(), 40);
    println!("{:?}", maps);
}