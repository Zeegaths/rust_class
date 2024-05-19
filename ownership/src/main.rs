fn main() {
    let a:i32 = 6;  //implicit clone- Both of them will print out
    let b:i32 = a;


    let mut name = "zee".to_string();
    let mut names = name; //pointer ownership changes

    names.push_str("gaths");
    
    println!("number {a}"); 
    println!("names {names}"); 

    let mut s1 = String::from("gathoni");
    let len = calculate_length(& mut s1);

    println!(" The length of {s1} is {len}");


    let mut  s2:String = "Wangui".to_string();
    let s3 = s2.push_str("my G");




    let c:String = String::from("Mary");
    //hello(c); //move
    let b = hello(c);
    println!("number {}", b);

    // let d = c;   

    // println!("Hello, world! {b} --{c} --{d}");

    //mutating references
    let mut s = String::from("Hello");

    change(&mut s);
    print!("{}", &mut s); //print does not insert a line at the end like println


    //mut borrowing
    let mut x = String::from("Jeex");

   { let y = &mut x;}
   let z = &mut x;

   println!("{}", z)



}



fn hello(param_1: String) -> String {
    println!("param 1 {}", param_1);
    return param_1;
}


fn calculate_length(s: & mut String) -> usize {
    s.len()    
}

fn change(a_string: &mut String) {
    a_string.push_str(" world");
}