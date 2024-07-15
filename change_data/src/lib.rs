use std::collections::HashMap;
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point <T>{
fn x(&self) -> &T{    
    &self.x
    }
}


 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point () {
        let integer_point = Point { x: 5, y: 10};
        let float_point_1 = Point { x: 1.0, y: 4.0};
        let float_point_2 = Point { x: "hey", y: "hello"};
        let float_point_3 = Point { x: "h", y: "hello"};

        assert_eq!(integer_point.x(), &5 );
        assert_eq!(float_point_1.x(), &1.0 );
        assert_eq!(float_point_2.x(), &"hey");
        assert_eq!(float_point_3.x(), &"h");
    }
}