// fn main() {
//     let x = Box::new(8);
//     println!("x is {}", x);

//     let x2 = *x * 10;
//     println!("x2 is {}", x2);
 
// }

//linked list/recursion
// use crate::List::{Cons, Nil};
// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }


// fn main () {
//     let list = Cons(1, Box::new(Cons(1, Box::new(Cons(2, Box::new(Nil))))));
//     // for i in list.iter(){
//         println!("{:?}",list);
//     // }
// } 

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self{
        Node {value, next: None}
    }

    fn append(&mut self, value: i32) {
        match self.next{
            Some(ref mut next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }
    //print the values in the list
    fn print(&self) {
        print!("{}", self.value);
        if let Some(ref next_node) = self.next {
            next_node.print();
        }
    }
}

fn main() {
    let mut head = Node::new(1);

    head.append(2);
    head.append(3);
    head.append(4);

    head.print();
}