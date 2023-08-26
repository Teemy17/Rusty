fn join_numbers(num: &[i64], separator: &str) -> String {
    let mut iter = num.iter();
    let mut result = String::new();


    if let Some(&n) = iter.next() {
        result.push_str(&format!("{}", n));

        for n in iter {
            result.push_str(separator);
            result.push_str(&format!("{}", n));
        }
    }

    return result
}

fn main() {
    let patterns = [6, 9, 4, 2, 0];
    let joined_comma = join_numbers(&patterns, ", ");
    println!("{}", joined_comma);
}


#[test]
fn test_join_numbers() { 
    assert_eq!(join_numbers(&[], ","), ""); 
    assert_eq!(join_numbers(&[25], ","), "25");
    let patterns = [5, 10, -1, 2]; 
    assert_eq!(join_numbers(&patterns, ", "), "5, 10, -1, 2"); 
    assert_eq!(join_numbers(&patterns, ";;"), "5;;10;;-1;;2");
}