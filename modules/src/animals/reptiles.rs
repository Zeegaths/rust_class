pub fn print_reptiles<'a>(list: &'a[&'a str]) -> &'a str {
    &list[1]
}


// pub fn print_reptiles<T>(list: &[T]) -> Option<&T> {    
//     if list.len() < 2 {
//         None
//     } else {
//         Some(&list[1])
//     }
   
// }