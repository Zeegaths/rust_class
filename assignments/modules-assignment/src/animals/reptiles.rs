//A function that takes in an array reference and returns the second element
pub fn print_reptiles<'a>(list: &'a[&'a str]) -> &'a str {
    &list[1]
}


//an alternative function that takes in a generic type and returns option
// pub fn print_reptiles<T>(list: &[T]) -> Option<&T> {    
//     if list.len() < 2 {
//         None
//     } else {
//         Some(&list[1])
//     }
   
// }