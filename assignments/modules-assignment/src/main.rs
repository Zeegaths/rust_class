//import the animals and mammals modules
mod animals;
mod mammals;
//gets the reptiles module from animals folder
use animals::reptiles;

//cals the functions from both modules
fn main() {
    let reptiles = ["snake", "crocodile", "turtle"];
    let result_1 = reptiles::print_reptiles(&reptiles);

    println!("The second result is {}", result_1);

    //calling te alternative function with generic input and option output
    // match result_1 {
    //     Some(reptile) => println!("The second reptile is {}", reptile),
    //     None => println!("There is no second reptile"),
    // }

    let my_mammal = mammals::print_mammals();
    println!("Total mammals:  {}", my_mammal);
}
