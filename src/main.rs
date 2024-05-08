
fn main() {
   //primitive data types
    //number to string
    let number_0 = 120;

    let mut number_string = number_0.to_string();

    number_string.push_str(" Get Together");

    println!("Let's {number_string}");

    //string to number
    let input_1 = "-64";
    let input_number: i32 = input_1.parse().expect("Not an integer");

    println!("Hey {}", input_number);



    //as keyword
    let number_2: i32 = 60;
    let number_3: i64 = number_2 as i64;


    
    //Secondary data types  
    //slices
    let (data, status, reason) = do_some_maths(45, 22);

    println!("sum = {data} was it success{status} reason: {reason}");


     //enums
     const age:i32 = 10;

     #[derive(Debug)] //attribute
     enum STATUS {
        ADULT,
        CHILD,
     }

     let user_status: STATUS = if age > 18 {
        STATUS::ADULT
     } else {
        STATUS::CHILD
     };

     println!("The status is {:?}",user_status)
}

fn do_some_maths(par_1:i32, par_2:i32) -> (i32, bool, String) {

    return(0, false, "execution failed".to_string())
}