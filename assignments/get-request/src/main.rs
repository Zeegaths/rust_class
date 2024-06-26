// extern crate futures;
// extern crate hyper;
// extern crate tokio_core;

// use std::io::{self, Write};
// use futures::{Future, Stream};
// use hyper::Client;
// use tokio_core::reactor::Core;


// fn main() {
//     let mut core = Core::new().unwrap();
//     let client = Client::new(&core.handle());

//     let uri = "http://httpbin.org/ip".parse().unwrap();
//     let work =
//         client.get(uri).and_then(|res| {
//             println!("Response: {}", res.status());

//             res.body().for_each(|chunk| {
//                 io::stdout()
//                     .write_all(&chunk)
//                     .map_err(From::from)
//             })
//         });
//     core.run(work).unwrap();
// }
// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{resp:#?}");
//     Ok(())
// }



fn main() {
    //htpp crate
    
}