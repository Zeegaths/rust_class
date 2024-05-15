use std::io;
fn main() {

    // reverse from 50 to 1
    let mut number = 50;
    loop {
        println!("number {number}");
        number -= 1;

        if number == 0 {
            break;
        }
    }


    

    //loops from 0 to the input
    let mut my_input = String::new();
    println!("Enter a number:");

    io::stdin().read_line(&mut my_input).expect("failed");
     
    let my_number:i32 = my_input.trim().parse().expect("Enter a valid integer");

    for element in 0..=my_number {
        println!("Looping through {element}")
    }



    //subtracts, adds, multiplies and divides the input
    let mut my_input = String::new();
    println!("Enter a number:");

    io::stdin().read_line(&mut my_input).expect("failed");
     
    let my_number:i32 = my_input.trim().parse().expect("Enter a valid integer");

    let sum = my_number + 2;
    let sub = my_number - 2;
    let mul = my_number * 2;
    let div = my_number / 2;

    println!("Sum {sum}");
    println!("Sub {sub}");
    println!("Multiplication {mul}");
    println!("Division {div}");
}

