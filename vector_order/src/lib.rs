//write a function that takes in a vector and returns a string
//if the vectir elemts are in ascending irder return ascending 
//ifthey are in descending order return descending
//else return mixed

pub fn check_order(vecta: Vec<i32>) -> String {
    if vecta.is_empty() || vecta.len() == 1 {
        return "ascending".to_string(); // Assuming single or empty vectors are considered ascending
    }

    let mut ascending = true;
    let mut descending = true;

    for i in 1..vecta.len() {
        if vecta[i] < vecta[i - 1] {
            ascending = false;
        }
        if vecta[i] > vecta[i - 1] {
            descending = false;
        }
    }

    if ascending {
        "ascending".to_string()
    } else if descending {
        "descending".to_string()
    } else {
        "mixed".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(check_order(vec), "ascending");
    }

    #[test]
    fn test_descending() {
        let vec = vec![5, 4, 3, 2, 1];
        assert_eq!(check_order(vec), "descending");
    }

    #[test]
    fn test_mixed() {
        let vec = vec![3, 1, 4, 2, 5];
        assert_eq!(check_order(vec), "mixed");
    }

    #[test]
    fn test_empty() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(check_order(vec), "ascending");
    }

    #[test]
    fn test_single_element() {
        let vec = vec![1];
        assert_eq!(check_order(vec), "ascending");
    }
}

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![5, 4, 3, 2, 1];
    let vec3 = vec![3, 1, 4, 2, 5];

    println!("{}", check_order(vec1)); // Should print "ascending"
    println!("{}", check_order(vec2)); // Should print "descending"
    println!("{}", check_order(vec3)); // Should print "mixed"
}
