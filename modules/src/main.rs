// mod product;
// use product::Book;
// // struct  User{
// //     name: String,
// //     age: i32,
// //     id_number: i32,
// // }
// // impl User {
// //     pub fn walk(&self) {
// //         println!("user {} is walking", &self.name)
// //     }
// //     pub fn eat(&self) {
// //         println!("user {} is walking", self.name)
// //     }
// //     pub fn is_adult(&self) -> bool {
// //         if self.age > 18 {
// //             return true;
// //         }
// //         false        
// //     }
// // }
 

// //  struct Book {
// //     pub name: String,
// //     category: String,
// //     status: bool

// //  }
// fn main() {
//     // let kenn = User{
//     //     name: String::from("Kenn"), 
//     //     age: 300,
//     //     id_number: 34,
//     // };
//     // let result = kenn.is_adult();
//     // println!("is kenn an adult: {}", result);

//     let cairo_book = Book::new(String::from("cairo_book"),String::from("leisure"), true);
//     println!("Availability: {}", cairo_book.if_available())
// }

// //calculate area of a rectangle(height, width) using knowledge of structs


mod area;
use std::result;

use area::Rectangle;
fn main () {
    let rect_1 = Rectangle::new(10, 10);
    let result = rect_1.area();
    println!("The area is {}", result);
}

