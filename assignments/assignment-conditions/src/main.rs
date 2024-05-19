use std::io;
fn main() {

    //a loop that prints areverse from 50 to 1
    let mut number = 50;
    loop {
        println!("number {number}");
        number -= 1;
        //breaks at 0
        if number == 0 {
            break;
        }
    }


    

    //a loop that prints from 0 to the input
    let mut my_input = String::new();
    println!("Enter a number:");

    io::stdin().read_line(&mut my_input).expect("failed");
     
    let my_number:i32 = my_input.trim().parse().expect("Enter a valid integer");

    for element in 0..=my_number {
        println!("Looping through {element}")
    }


    //subtracts, adds, multiplies and divides the input
    //takes in user input
    let mut my_input = String::new();
    println!("Enter a number:");

    //reads the user input as string
    io::stdin().read_line(&mut my_input).expect("failed");
    //converts input to integer     
    let my_number:i32 = my_input.trim().parse().expect("Enter a valid integer");
    //adds, subtracts, devides
    let sum = my_number + 2;
    let sub = my_number - 2;
    let mul = my_number * 2;
    let div = my_number / 2;
    //prints out the results
    println!("Sum {sum}");
    println!("Sub {sub}");
    println!("Multiplication {mul}");
    println!("Division {div}");
}

