fn make_arrow(n: i64) -> String {
    let mut result = String::new();
    
    for i in 0..n {
        for _j in 0..i + 1 {
            result.push('*');
        }
        result.push('\n');
    }

    for i in 0..n {
        for _j in 0..n - i - 1 {
            result.push('*');
        }
        result.push('\n');
    }
    result
}

fn make_arrow2(n: i64) -> String {
    let mut result = String::new();

    for _i in (1..n).rev() {
        for _j in 1..=_i {
            result.push(' ');
        }
        for _l in _i..n {
            result.push('*');
        } 
            result.push('\n');    
        
    }

    for _i in 0..n {
        for _j in 0.._i {
            result.push(' ');
        }
        for _k in _i..n {
            result.push('*');
        } 
            result.push('\n');
    }
   result
} 



#[test]
fn test_make_arrow() {
    assert_eq!(make_arrow(3), "*\n**\n***\n**\n*\n\n");
    assert_eq!(make_arrow(4), "*\n**\n***\n****\n***\n**\n*\n\n");
    assert_eq!(make_arrow(5), "*\n**\n***\n****\n*****\n****\n***\n**\n*\n\n");
}

#[test]
fn test_make_arrow2() {
    assert_eq!(make_arrow2(3), "  *\n **\n***\n **\n  *\n");
    assert_eq!(make_arrow2(4), "   *\n  **\n ***\n****\n ***\n  **\n   *\n");
    assert_eq!(make_arrow2(5), "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n");
}

