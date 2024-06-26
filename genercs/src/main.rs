// mod modname {
//     use std::fmt::Debug;

    // pub(crate) fn main() {
    //     println!("Hello, world!");
    //     // println!("largest number {}", result);
    //     let name = "mary";
    //     let reslt = print_debug(name);
    //     println!("largest number {:?}", reslt);

    //     let arr = [1, 2, 3, 4, 5];

//    let culture_one = Culture{name: "Raggae".to_string()};
//    println!{"{:?}", culture_one}
    //     largest_no = largest_integer(&arr);
    //     println!("largest number {}", largest_no);
            
    // }

//     //integers
//     pub(crate) fn largest_integer (list : &[i32])  -> i32{
//         let mut largest = list[0];
//         for & i in list.iter() {
//             if i > largest {
//                 largest = i;
//             }
//         }
//         largest
//     }

//     //characters
//     pub(crate) fn largest_character (list : &[char])  -> char{
//         let mut largest = list[0];
//         for & i in list.iter() {
//             if i > largest {
//                 largest = i;
//             }
//         }
//         largest
//     }

//     //generic function
//     pub(crate) fn largest<Z: PartialOrd + Copy> (list : &[Z])  -> Z{
//         let mut largest = list[0];
//         for & i in list.iter() {
//             if i > largest {
//                 largest = i;
//             }
//         }
//         largest
//     }

//     pub(crate) fn print_debug<T : Debug>(item: T) {
//         println!("{:?}", item)
// fn print_display<T: Display> (item: T) {
//     println("{}", item)
// }
//     }
// // }

// enum AccountTypes<Y, X, Z> {
//     ADMIN(Y),
//     SUDO(X),
//     NORMAL(Z)
// }

// fn main() {
//     let one = AccountTypes::ADMIN(3.5);
//     let two = AccountTypes::NORMAL("ZEE".to_string());

//     match one {
//         AccountTypes::NORMAL(value)=>println!("The value is {}", value),
//         AccountTypes::ADMIN => println!("Culture is two{}", value)
//     }
// }

// struct Point<D>() {
//     x: D,
//     y:D,
//     z: i32
// }

// impl <D> Point<D> {
//     fn new(x:D,y:D, z:i32) -> Self{
//         Point{x, y,z}
//     }
//     fn get_x(&self) ->&D{
//         &self.x
//     }
// };
// fn main () {
    // let new_value = Point::new(3.5, 5.6);
    // println!("x is at position {}", my_new_one.get_x())

// }

// fn swap<T>(x: &mut T, y: &mut T) {
//     std::mem::swap(x, y);
// }

// fn main() {
//     let mut a = 1;
//     let mut b = 2;
//     swap(&mut a, &mut b);
//     println!("a = {}, b = {}", a, b);
// }
// #[derive(Debug)]
// struct Vehicle<T, X>{
//     customer_one: T,
//     customer_two: X
// }

// impl <T, X> Vehicle<T,X>{
//     fn new(customer_one: T, customer_two: X) -> Self {
//         Vehicle{customer_one, customer_two}
//     }

//     fn swap(& mut self, other_client: Vehicle<T, X>) {
//         self.customer_one = other_client.customer_one;
//         self.customer_two = other_client.customer_two;
//     }
// }
// fn main() {
//     let mut x = Vehicle::new("KAZ", "KBD".to_string());
//     let y = Vehicle::new("KDD", "KCC".to_string());
//     x.swap(y);
//     println!("x = {:?}", x);
// }

use std::fmt::Debug;
fn something<X:Debug, Y:Debug, Z:Debug>(one: X, two:Y, three:Z) {
    println!("Parameter two {:?}", two);
}
fn main() {
    something(30, "Zee".to_string(), 50)
}

// struct School<S>{
//     name: S,
//     population: S
// }

// impl<S>School<S>{
//     fn new{name:S, population:S}-> Self{
//         School{name, population}
//     }
// }

