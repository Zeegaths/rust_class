fn main() {
    // println!("Hello, world!");
    let var_1 = [4, 2, 3, 5];

    let summ = calculate_sum(&var_1);
    println!("sum is {summ}");


    let var_2 = [0.1, 0.5, 8.9];
    
    let summ2 = calculate_sum2(&var_2);
    println!("sum 2 is {summ2}");

    let mut my_string = String::from("Good");
    
    let greeting = mutate_string(& mut my_string);
    print!("Greeting {greeting}");
}

//takes in array- reference
//loops through items
//returns sum

fn calculate_sum(param_1: &[u32]) -> u32{

    let mut param_2 = 0;
    for elements in param_1 {
        param_2 = elements + param_2;        
    }
    param_2        
    
}


fn calculate_sum2(par_1: &[f32]) -> f32{

    let mut index = 0;
    let length = par_1.len();
    let mut totals =0.0;
    
    while index < length {
        index = index + 1;
        totals = totals + par_1[index-1];
    }

    return totals
   
    
}

fn mutate_string (string_1: & mut String) -> &String{
    string_1.push_str("Evening");
    string_1

}