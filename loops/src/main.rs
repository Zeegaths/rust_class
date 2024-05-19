fn main() {
//    let x: u32 = 5;
   let a: f64 = -4.5;
//    let mult = 3 * 8;
   let _c = 'z';
   let cc: char = 'X';

   let f:[i32; 5] = [-2, 3, 4, 5, 10];

   let tup:(i32, f32, u16) = (400, 6.4, 1);
   let(x, y, z) = tup;
 
   println!("the value of x is: {x}");
   println!("the value of y is: {y}");
   println!("the value of z is: {z}");

   let addy = add(4, 5);
   let multy = mult(6, 7);

   let condition = true;
   let number = if condition{5} else {6};



  fn rloop() {
    let mut counter = 0;
    loop {
        println!("Hello World");

        if counter == 5 {
            break;
        }
    }
   }}


fn add(x: u32, y:u32) -> u32{
    x + y
}


fn mult(x: u32, y:u32) -> u32{
    return x * y;
}

fn while_loop (){
    let mut num = 0;
    while num <4 {
        println!("{num}");
        num += 1;
    }
}
