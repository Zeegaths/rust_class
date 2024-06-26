// trait Describe {
//     fn describe_user(&self)->String;

//     fn describe(&self) {
//         println!("This is a description")
//     } 
// }

// struct Person {
//     name: String,
//     age: i32
// }


// impl Describe for Person {
//     fn describe_user(&self)->String{
//         format!("user name:{} Age: {}", self.name, self.age)
//     }   
//     fn describe(&self) {
//         println!("This is another description")
//     } 
// }
// fn main() {
//     let p = Person{
//         name: String::from("Zee"),
//         age: 20
//     };
//     println!("{}", p.describe_user());
//     println!("{:?}", p.describe());

// }



use std::f64::consts::PI;

trait Shape{
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
struct Rectangle {
    height: f64,
    width: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
fn myFunction(my_input: impl Shape) {
    println!("This function takes in a trait")
}

fn main() {
    let circle = Circle{radius : 6.2};
    let rectangle = Rectangle{height: 3.2, width: 4.0};
    println!("The area of the circle is: {}", circle.area());
    println!("The area of the circle is:: {}", rectangle.area());

    let res = myFunction(circle);
    println!("The implementation is a circle{:?}", res)

}
