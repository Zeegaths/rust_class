fn main() {
    let name = "Mary";
    let number = 24;
   
    //signed and unsigned integers
    let no1: i8 = 10;
    let no2: u8 = 11;

    let no3: i16 = 12;
    let no4: u16 = 13;

    let no4: i32 = 14;
    let no5: u32 = 15;

    let no6: i64 = 14;
    let no7: u64 = 15;

    let no8: i128 = 14;
    let no9: u128 = 15;

    let no10: isize = 14;
    let no11: usize = 15;

    let character : char = 's';

    let float:f32 = 0.5;


    const text1: &str = "Welcome to Programming";
    let text_2 :String = String::from("Hello....");

    let age_2 = {
        12 +6
    };

    //calling the functions
    sum(6, 8);
    sum2(10, 20);
    sum3(15, 5);


    println!("float is {}", float);
    println!("age is {}", age_2);
}

//funtion without return value
fn sum(number_1: i32, number_2: i32) {
    let number_3 = number_1 + number_2;
    println!("Sum of 1 and 2 {}", number_3);
}

//function with return
fn sum2(number_1: i32, number_2: i32)-> i32 {
    let number_3 = number_1 + number_2;
    println!("Sum of 1 and 2 {}", number_3);
    return number_3;
}

//explicit declaration
fn sum3(number_1: i32, number_2: i32)-> i32 {
    let number_3 = number_1 + number_2;
    println!("Sum of 1 and 2 {}", number_3);
    number_3
}