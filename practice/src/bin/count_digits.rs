fn count_digits(input: &str) -> usize {
    let mut count = 0;
    
    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
    return count
}

fn main() {
    let input = vec![1.0, -2.5, 3.7, -4.0, 5.2];
    let non_negatives = extract_non_negatives(&input);

    println!("Original list: {:?}", input);
    println!("Non-negative numbers: {:?}", non_negatives);
}

fn count_vowels(input: &str) -> usize {
    let mut count = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    for c in input.chars() {
        if vowels.contains(&c) {
            count += 1
        }
    }
    return count
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
