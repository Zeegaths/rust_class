fn main() {
    
   
    let x = longer("par_1", "par_2 test_");
    println!("x = {x}");
    
}


fn longer<'a>(par_1 : &'a str, par_2: &'a str) -> &'a str{
    if par_1.len() > par_2.len() {
        par_1
    }else{
        par_2
    }
}