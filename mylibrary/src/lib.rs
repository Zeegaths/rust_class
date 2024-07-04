use std::fmt::Display;
use std::ops::Div;
use num::Zero;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn say_hello(x: String) -> String{
    format!("{} Kamau", x)
}

    //early return
    //sonar lint
pub fn should_panic (number: i32) {
    if number > 100 {
        panic!("Too large")
    }

    if number < 50 {
        println!("You are almost there");
        return;
    }
    println!("just reduce it by a little")
} 

//cant divide by 0
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Number cannot be divided with zero"))
    } else {
        Ok(a/b)
    }
}
//Generic implementation 

fn division<T>(a: T, b: T) -> Result<T, String>
    where

    T: Zero + Div<Output = T> + Display + Copy + PartialOrd,
    {

    if b == T::zero() {

    Err(format!("Cannot divide by zero"))    

    } else {    

        Ok(a / b)
    }
}

// Write a Rust function that takes a tuple (char, i32) and returns "Vowel" if the first element is a vowel ('a', 'e', 'i', 'o', 'u') and "Consonant" otherwise.
fn check_character(char_input: (char, i32)) -> &'static str{
    match char_input.0{
        'a'|'e'|'i'|'o'|'u' => "vowel",
        _=>"Constants",
    }
}

fn check_alphabets(input: (char, i32)) -> String{
    let my_vec = vec!['a','e','i', 'o', 'u'];

    if my_vec.iter().any(|t| *t == input.0) {
        "vowel letter".to_string()
    }else{
        "constant".to_string()
    }
}

fn classify_booleans(vec: Vec<bool>) -> &'static str {
    if vec.iter().all(|x| *x) {
        "All true!"
    } else if vec.iter().all(|x| !x) {
        "All false!"
    } else {
        "Mixed!"
    }
}

// create a generic function that takes two paramter and compares the two parameters returning the less/smaller of the two
pub fn get_smaller <T: std::cmp::PartialOrd> (x: T, y: T)-> T {
    if x < y {
        x
        // println!("The smaller number is {}", x);
    } else {
        y
        // println!("The smaller number is {}", y);
    }

    
} 
fn main() {
    let char_result = vec![('a', 1), ('o', 2), ('u', 3)];
    for i in char_result{
        println!("{}", check_character(i));
    }

    let bool_vec1 = vec![true, true, true];
    let bool_vec2 = vec![false, false, false];
    let bool_vec3 = vec![true, false, true];

    println!("Vector 1: {}", classify_booleans(bool_vec1));
    println!("Vector 2: {}", classify_booleans(bool_vec2));
    println!("Vector 3: {}", classify_booleans(bool_vec3));
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 10); //not equal
    }
    #[test]
    fn test_contains() { 
        assert!(add(2,2) == 4);
        let res = say_hello("kenn".to_string());
        assert!(res.contains("kenn"));
    }
    
    #[test]
    #[should_panic]
    fn should_panic_test() {
        should_panic(500);
    }

    #[test]
    fn test_divide() {
        let result = divide(10.0, 5.0);
        assert_eq!(result, Ok(2.0))
    }


    #[test]
    fn test_wrong() {
        let result2 = divide(10.0, 0.0);
        assert_eq!(result2, Err(String::from("Number cannot be divided with zero")))
    }

    #[test]
    fn test_char() {
        
        let result2= get_smaller("a", "b");
        assert_eq!(result2, "a")
    }


    #[test]
    fn test_booleans() {    
        let bool_vec1 = vec![true, true, true];
        let bool_vec2 = vec![false, false, false];
        let bool_vec3 = vec![true, false, true];
       
          
        let result2= classify_booleans(bool_vec1);
        let result3 = classify_booleans(bool_vec2);
        let result4 = classify_booleans(bool_vec3);
        assert_eq!(result2, "All true!");
        assert_eq!(result3, "All false!");
        assert_eq!(result4, "Mixed!");
    }

}
