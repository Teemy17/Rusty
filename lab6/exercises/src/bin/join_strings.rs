fn join_strings(strings: &[&str], separator: &str) -> String {
        if strings.is_empty() {
        return String::new();
    }

    let mut result = strings[0].to_string();
    for i in 1..strings.len() {
        result.push_str(separator);
        result.push_str(strings[i]);
    }

    return result
}

fn main() {
    let patterns = ["C", "Rust", "C++", "Python"];
    let joined_comma = join_strings(&patterns, ", ");
    println!("{}", joined_comma);
}


#[test]
fn test_join_strings() { 
    assert_eq!(join_strings(&[], ","), ""); 
    assert_eq!(join_strings(&["C"], ","), "C");

    let patterns = ["C", "Rust", "C++", "Python"]; 
    assert_eq!(join_strings(&patterns, ", "), "C, Rust, C++, Python"); 
    assert_eq!(join_strings(&patterns, ";;"), "C;;Rust;;C++;;Python");
}   