use std::{sync::mpsc::{self, Receiver}, thread, time::Instant};

use std::sync::Mutex;
use std::sync::Arc;

//Using a mutex

fn sum_mutex(vec:Vec<i32>) -> i32 {
    let sum_main = Arc::new(Mutex::new(0));

    let chunk_size  = vec.len()/ 4;
    let mut vec_handle = Vec::new();

    for chunk in vec.chunks(chunk_size) {
        let sum: Arc<Mutex<i32>> = Arc::clone(&sum_main);

        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            let my_sum:i32 = chunk.iter().sum();

            let mut sum = sum.lock().unwrap();

            *sum+=my_sum
        });
        vec_handle.push(handle);
    }
    for handle_item in vec_handle {
        handle_item.join().unwrap()
    }

    *sum_main.lock().unwrap()

}

//multiple producers single channel
fn sum_mpsc(vec: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();

    let mut vec_handle = Vec::new();
    let chunk_size = vec.len() / 4;

    let mut sum = 0;

    println!("test 1");
    for chunk in vec.chunks(chunk_size) {
        let tx = tx.clone();
        let chunk = chunk.to_vec();

        println!("test {:?}", chunk);
        let handlle = thread::spawn(move || {
            let my_sum:i32 = chunk.iter().sum();
            tx.send(my_sum).unwrap()
        });
        vec_handle.push(handlle);
    }
    drop(tx);

    for recieved in rx {
        println!("text {recieved}");
        sum += recieved
    }
    sum

}

fn sum_via_iter(vec: Vec<i32>) -> i32{
    vec.iter().sum()
}

fn main() {
    //large vec
    let v: Vec<i32> = (1 ..=10_000).collect();

    let start1 = Instant::now();
}
