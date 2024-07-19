
macro_rules! Hello_world {
    () => {
        println!("hello world")
    };
}

macro_rules! hello_too {
    ($message:expr) => {
        println!("hello world {}", $message)        
    };
}

//one parameter
macro_rules! multiply_by_five {
    ($par:expr) => (10* $par)      
    
}

//multiple parameters
macro_rules! print_something {
    ($($y:expr), *) => {
        $(
            println!("{}", $y);
        )*        
    };
}
// fn main() {
//     Hello_world!();
//     hello_too!(99);
//     hello_too!("You are amazing");
//     print_something!(1,2,3,4,5);
//     print_something!("my", "name", "is", 23);
//     assert_eq!(25, multiply_by_five!(2 + 3))  

// }

//logs
fn get_log_level() ->i32 {
    1
}
macro_rules! log {
    ($msg:expr) => {{
        let state = get_log_level();
        if state > 0 {
            println!("DEBUG :: {}", $msg)
        }
    }          
    };
}

fn main() {
    log!("Main started")
}