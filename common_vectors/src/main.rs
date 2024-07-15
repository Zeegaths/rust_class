fn main() {
    let one = vec![1,2,3,4];
    let two = vec![6,2,5,4];
    let three = common_vectors(one, two);

    println!("{:?}", three)
}


// Write a Rust function that takes two vectors as 
// input and returns a new vectors containing 
// elements that are common to both input vectors
use std::fmt::Debug;

fn common_vectors<T: Clone + PartialEq + Debug + PartialOrd>(vec_1: Vec<T>, vec_2:Vec<T>) -> Vec<T>{

    let mut vec_3: Vec<T> = Vec::new();
    
    for item in vec_1 {

        let exists = vec_2.contains(&item);
        if exists {
            vec_3.push(item);
        }
    }
    vec_3

}