use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};
use std::rc::Rc;


// fn main() {
//     let x = Rc::new(RefCell::new(String::from("My namee is Zee")));
//     let shared1 = Rc::clone(&x);
//     let shared2 = Rc::clone(&x);

//     shared1.borrow_mut().push_str("multiple owners");
//     println!("shared {}", shared2.borrow())  
// }


//mutex single thread
// fn main() {
//     let x = Mutex::new(10);
//     {
//         let mut my_data =x.lock().unwrap();
//         *my_data=33;
//     }

//     println!("my data is {:?}", x)
// }




//mutex multi thread
// fn main() {

//     let x = Arc::new(Mutex::new(10));

//     for i in 0..10 {
//         let x_clone = Arc::clone(&x);
//         thread::spawn(move|| {            
//             let mut my_data = x_clone.lock().unwrap();
//             *my_data = i;
//         });
//     }

//     println!("my data is {:?}", x.lock().unwrap());

//     for i in 0..100 {
//         thread::sleep(Duration::from_millis(i));
//     }
// }


//multiple threads
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10{
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move||{
            let mut mu_var = counter.lock().unwrap();
            *mu_var += 1

        });
        handles.push(handle);
        
    }
    for i in handles {
        i.join().unwrap()
    }

    println!("Result {}", counter.lock().unwrap())
}