// fn main() {
//     let square = |x: i32| -> i32 {
//         x*x
//     };

//     let sample = ||->(){
//         print!("test")
//     };
//     let result = square(4);

//     println!("Result is {}", result);
//     sample()
    
// } 



fn main(){
    let test = 3;

    let multiply = |x:i32| -> i32{
        x*test
    };
    let res = multiply(10);

    println!("Result is {}", res);

    let age = "zee".to_string();
    let display_age = move || {
        println!("Your age is {}", age)
    };

    let multiple = |x:i32| -> i32{
        x * 3
    };

    //filter to find even numbers
    let numbers = vec![1,2,3,4,5,6]; 
    let result = numbers.into_iter().filter(|x |{x%2 == 0}).collect();

    //find number greater that 5
    let result2 = numbers.iter().find(|x |**x == 5).unwrap();

    let result = multiply_by_three(multiple);
    println!("The multiple is {}", result);

    //closure as a parameter

    fn multiply_by_three<K>(param_1:K)->i32 where K:Fn(i32) -> i32 {
        param_1(10)

    }

    fn create_multiples(factor: i32) -> impl Fn(i32) -> i32 {
        move |i| {
            i * factor
        }
    }

    //box move the item from stack to  heap
    fn create_multiples(factor: i32) -> Box<dyn Fn(i32) -> i32{
        Box::new(|x| x + 1);
    }

   

    

}