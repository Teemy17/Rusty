fn count_digits(input: &str) -> usize {
    
    let mut count = 0;
    
    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
return count

}

fn count_digits_r(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    let char = input.chars().next().unwrap();
    if char.is_digit(10) {
        return 1 + count_digits_r(&input[1..]);
    } else {
        return count_digits_r(&input[1..]);
    }
}

fn count_digits_v2(input: &str) -> Vec<(String, usize)> {
    let mut word_count = Vec::new();
    for word in input.split_whitespace() {
        let mut count = 0;
        for c in word.chars() {
            if c.is_digit(10) {
                count += 1;
            }
        }
        word_count.push((word.to_string(), count));
    }
    return word_count
}

fn extract_non_negatives( input: &[f64]) -> Vec<f64> {
    let mut non_negative = Vec::new();
        for &num in input {
            if num >= 0.0 {
                non_negative.push(num)
            }
        }
    return non_negative
}

fn split_non_negatives( input: &[f64]) -> (Vec<f64>, Vec<f64>) {
     let mut pos = Vec::new();
     let mut neg = Vec::new();
        for &num in input {
            if num >= 0.0 {
                pos.push(num);
            }
            else if num < 0.0 {
                neg.push(num);
            }
        }
    return (pos, neg)
}

fn extract_non_negatives_r( input: &[f64]) -> Vec<f64> {
   let mut non_negative = Vec::new();
    if input.is_empty() {
        return non_negative;
    }
    else if input[0] >= 0.0 {
        non_negative.push(input[0])
    }
    non_negative.extend(extract_non_negatives_r(& input[1..]));
    
    return non_negative
}



    

#[test]
fn test_digits_count1() {
    assert_eq!(count_digits(""), 0);
    assert_eq!(count_digits("abcd"), 0);
    assert_eq!(count_digits("ab12xy5 7x83y5z"), 7);
    assert_eq!(count_digits("weedday69d420"), 5);
}

#[test]
fn test_digits_count1_r() {
    assert_eq!(count_digits_r(""), 0);
    assert_eq!(count_digits_r("abcd"), 0);
    assert_eq!(count_digits_r("ab12xy5 7x83y5z"), 7);
    assert_eq!(count_digits_r("weedday69d420"), 5);
    assert_eq!(count_digits_r("334454321234567890987654321"), 27);
}

#[test]
fn test_digits_count2() {
    assert_eq!(count_digits_v2(""), []);
    assert_eq!(
    count_digits_v2("ab12xy5 7x83y5z"),
        [
    ("ab12xy5".to_string(), 3), // '1', '2', '5'
    ("7x83y5z".to_string(), 4) // '7', '8', '3', '5'
        ]
    );
     assert_eq!(
    count_digits_v2("sm0k369weed420 d4rf42fg50"),
        [
    ("sm0k369weed420".to_string(), 7), 
    ("d4rf42fg50".to_string(), 5) 
        ]
    );
}

#[test]
fn test_extract_non_negatives() { 
    assert_eq!(extract_non_negatives(&[]), []); 
    assert_eq!(extract_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),[0.8, 1.6, 10.5]);
}

#[test]
fn test_split_non_negatives() { 
    assert_eq!(split_non_negatives(&[]), (vec![], vec![])); 
    assert_eq!(split_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]), (vec![0.8, 1.6, 10.5], vec![-5.1, -6.5] )); 
}

#[test]
fn test_extract_non_negatives_r() { 
    assert_eq!(extract_non_negatives_r(&[]), []); 
    assert_eq!(extract_non_negatives_r(&[0.8, -5.1, 1.6, -6.5, 10.5]),[0.8, 1.6, 10.5]);
}