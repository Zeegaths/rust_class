// fn main() {
<<<<<<< HEAD
//     // let x = vec![1, 2, 3, 4, 5];

//     // #[doc = "This is the first solution to looping through the `vector`"]
//     // for i in x {
//     //     println!("Looping through {i}");
//     // }

//     // let mut i = 0;
//     // while i < x.len() {
//     // println!("The number is {:?}", x[i]);
//     // i += 1
//     // }

//     let mut x = Vec::new();
//     x. push(1);
//     x. push(5);
//     x. push(3);

//     println!("before {:?}", x);
//     let finals = x.len().saturating_add(2);
//     x.push(5);
//     x.push(6);
//     x.push(7);
    
//     x.truncate(finals);

//     println!("after {:?}", x);



//     // for i in x {
//     //     let mut y = i + 1;

//     //     println!("Looping through {i} and {y}");
//     // }

//     // for i in x.iter_mut() {
//     //     *i += 10;
//     // }
//     // println!("mutated {:?}", x)
// // } 



// let date_time_string = "2024-05-23T12:34:56";
// // let date: Vec<&str> = date_time_string.split('T'). collect();
// // println!("The time is{:?}", date[1]);
// let index = date_time_string.find('T').expect("T not found");
// let(date, time) = date_time_string.split_at(index);
// let time_part = &time[1..];

// println!("index of 'T' {}",index);
// print!("Time part: {}", time_part);

// }

// fn main() {
    // let my_string = "My name is Mary".to_string();
    // let res = match my_string.chars().nth(10) {
    //     some(c) => c.to_string();
    //     none => 
    // }

    // let res = print_occupation("Scientist");
    // println! ("{}", res.expect("message anavailable"));

    // fn print_occupation(name : &str) -> Option<&str> {
    //     match name {
    //         "programmaer" => Some("Hey fellow coder"),
    //         "mathematician" => Some("numbers huh?"),
    //         _=>None
    //     }
    // }
// }
// use std::io;
// fn main() {

//     let mut my_input = String::new();
//     println!("Enter a number:");

//     io::stdin().read_line(&mut my_input).expect("failed");
     
//     let my_number:i32 = my_input.trim().parse().expect("Enter a valid integer");
//     res = print_number(my_number);
//     println! ("{}", res.expect(0));

//     let res = print_number("Scientist");
//     println! ("{}", res.expect("message anavailable"));

//     fn print_number(number : &str) -> Option<&i32> {
//         match number {
//             => Some(number),
//             "mathematician" => Some("numbers huh?"),
//             _=>None
//         }
//     }
// }

// fn main() {
//     let vect_1 =vec! [1, 2, 3, 2, 5, 6];
//     let vect_2 = 2;
//     // let res = my_function(vect_1, vect_2);
//     // println!("This is my vector {:?}", res);
//     // println!("{}", res.expect("Not found"));

//     match my_function(vect_1, vect_2){
//         Some(value) => println!("{}", value),
//         None => println!("Not Found"),
//     };
// }

// fn my_function(param_1: Vec<i32>, param_2: usize) -> Option<i32>{
//     // checks if length of param1 is less than param 2
//     if param_1.len() > param_2{
//         println!("length of param1 is less than param2");
//         Some(param_1[param_2])

//     }else {
//         return None;
//     }
// }

// //results
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//     let greeting_file = match greeting_file_result{
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!{"Problem creatung the file: {:?}", e},
//             },
//             other_error => {
//                 panic!();           
    
// }

// fn  main() {
    
// }
 
// fn get_contents_of_file() -> Result<String, io::Error> {
//     Ok
// }


//Hashmaps

// use std::collections::HashMap;
// fn main () {
//     let mut maps: HashMap<String, i32>= HashMap::new();
//     maps.insert("Hello".to_string(), 22);
//     maps.insert("world".to_string(), 30);
//     maps.insert("Nigeria".to_string(), 34);

//     println!("{:?}", maps);

//     maps.insert("Hello".to_string(), 40);
//     println!("{:?}", maps);

//     for key in maps.keys() {
//         println!("{key}");
//     }

//     maps.entry("Kenya".to_string()).or_insert(500);
// }



// a struct with vectors and hasmaps
//call a function of the struct to update the vector
////call a function of the struct to udate the hashmap
//
//
// use std::collections::HashMap;

// struct Exams {
//     pub number: Vec<i32>,
//     pub score : HashMap<String, i32>
// }

// impl Exams {
//     pub fn new(param_1: Vec<i32>, param_2 :HashMap<String, i32> ) -> Self{
//         Exams{
//             number: param_1,
//             score: param_2
//         }
//     }
//     pub fn add_numbers(&mut self, param_1:i32){
//         self.number.push(param_1);
//     }

//     pub fn add_score(&mut self, param_1: String) {
//         self.score.entry(param_1).or_insert(0);
//     }
// }   
// fn main() {
//     let mut exam_scores = Exams::new(Vec::new(),
// HashMap::new());

//     exam_scores.add_numbers(10);
//     println!("random number b4 {:?}", exam_scores.number);

//     exam_scores.add_numbers(20);
//     exam_scores.add_numbers(30);
//     println!("random number after {:?}", exam_scores.number);

//     exam_scores.add_score("Zee".to_string());
//     println!("Scores before{:?}", exam_scores.score);

//     exam_scores.add_score("Tim".to_string());
//     println!("Scores after{:?}", exam_scores.score);

// }
 
 //Storing different data types in a vector using enums


//  fn main() {

//     #[derive(Debug)]
//     enum SpreadSheet{
//         Int(i32),
//         Float(f32),
//         Text(String)
//     }

//     let row = vec![
//         SpreadSheet::Int(3),
//         SpreadSheet::Float(30.5),
//         SpreadSheet::Text("Mama I made it".to_string())
//     ];

//     let x = row.get(4).expect("NOT FOUND");
//     println!("{:?}", x)

//  }

//http library- cargo- perfom a get request to any url
//look at options and results
=======
    // let  mut x = vec![1, 2, 3, 4, 5];

    // #[doc = "This is the first solution to looping through the `vector`"]
    // for i in x {
    //     println!("Looping through {i}"x[i]);
    // }

    // let mut i = 0;
    // while i < x.len() {
    // println!("The number is {:?}", x[i]);
    // i += 1
    // }

    // let mut x = Vec::new();
    // x. push(1);
    // x. push(5);
    // x. push(3);

    // for i in x.iter_mut() {
    //     *i += 10
    // }

    // for i in x {
    //     let mut y = i + 1;

    //     println!("Looping through {i} and {y}");
    // }
// } 


// let date_time_string = "2024-05-23T12:34:56";
// let date: Vec<&str> = date_time_string.split('T'). collect();
// println!("The time is{:?}", date[1]);

// }

//Hashmaps

use std::collections::HashMap;
fn main () {
    let mut maps: HashMap<String, i32>= HashMap::new();
    maps.insert("Hello".to_string(), 22);
    maps.insert("world".to_string(), 30);
    maps.insert("Nigeria".to_string(), 34);

    println!("{:?}", maps);

    maps.insert("Hello".to_string(), 40);
    println!("{:?}", maps);
}
>>>>>>> 92bb826016c5f29c563cc2f8c55e8c06d0062d89
