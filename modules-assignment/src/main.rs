mod animals;
mod mammals;

use animals::reptiles;

fn main() {
    let reptiles = ["snake", "crocodile", "turtle"];
    let result_1 = reptiles::print_reptiles(&reptiles);

    println!("The second result is {}", result_1);

    // match result_1 {
    //     Some(reptile) => println!("The second reptile is {}", reptile),
    //     None => println!("There is no second reptile"),
    // }

    let my_mammal = mammals::print_mammals();
    println!("Total mammals:  {}", my_mammal);
}
