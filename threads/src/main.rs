// use std::thread;
// use std::time::Duration;
// fn main() {
//     let handle = thread::spawn(||{
//         println!("Hello there")
//     }); 
//     handle.join().unwrap();  
//     println!("hi youu");
//     // for i in 0..10 {
//     //     thread::sleep(Duration::from_millis(1))
//     // }
// }

//using move for elements imported into the handle
// use std::thread;
// fn main() {
//     let v= vec![1,2,3,4];

//     let handle = thread::spawn(move||{
//         println!("Hello {:#?}", v)
//     }); 

//     handle.join().unwrap();   
  
// }


//multiple threads/ moves
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
// fn main() {
//     let (tx, rx) = mpsc::channel(); 

//     for i in 0..10{
//         let tx_clone = tx.clone();
//         thread::spawn( move||{
//             tx_clone.send(format!("loop number {}", i));
//             thread::sleep(Duration::from_millis(10));
//         });
//     }

//     for r in rx{
//         println!("received {r}")
//     }
// }

//single thread
fn main() {
    let (tx, rx) = mpsc::channel(); 

    
        // let tx_clone = tx.clone();
        thread::spawn( move||{
            for i in 0..10{
                tx.send(format!("loop number {}", i));
            } 
        });
    

    for r in rx{
        println!("received {r}")
    }
}