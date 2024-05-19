fn main() {
    // println!("Hello, world!");

    // let a = 20;
    // let b = 200;

    // if a > b {
    //     println!("{a} is greater than {b}")
    // }else if a == b {
    //     println!("{a} is equal to {b}") //you can leave out the semicolon if its just one line on the block
    // } else {
    //     println!("{a} is less than {b}")
    // }

    //if statements
    let network_response = 200;
    let is_success: bool = if network_response == 200 { true } else { false };
    if is_success {
        println!("Operation was successful")
    }
    let mut number = 0;
    loop {
        println!("I am number {number}");
        number += 1;

        if number == 1 {
            println!("Loop starts");
            continue;
        }

        if number == 5 {
            println!("damn");
            break;
        }
    }

    //while loops
    let mut number = 5;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("GO!");


    //arrays
    let array = [1, 2, 3, 4, 5, 6, 7]; //arrys are known at compile time
    let mut index = 4;
    while index != 0 {
        println!("looping through index{index} item in array {}!", array[index]);
        index -=1;
    }


    //for in
    for element in array{
        println!("Looping through {element}")
    }
     
     //arrange
    for element in 1..5{
        println!("Looping through {element}")
    }
    
}
