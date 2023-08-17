fn count_vowels(input: &str) -> usize {
    let mut count_vowels = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    
    for chars in input.chars() {
        if vowels.contains(&chars) {
            count_vowels += 1;
        }
    }
    return count_vowels
}


fn count_vowels_rec(input: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    
    if input.is_empty() {
        return 0;
    }
    let char = input.chars().next().unwrap();
    if vowels.contains(&char) {
        return 1 + count_vowels_rec(&input[1..]);
    } 
    else {
        return count_vowels_rec(&input[1..]);
    }
}


fn count_vowels_v2(input: &str) -> Vec<(String, usize)> {
    let mut letter_vowels = Vec::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    
    for letter in input.split_whitespace() {
        let mut count_vowels = 0;
        for chars in letter.chars() {
            if vowels.contains(&chars) {
                count_vowels += 1;
            }
        }
        letter_vowels.push((letter.to_string(), count_vowels));
    }
    return letter_vowels
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("An eagle is outer under"), 10);
}

#[test]
fn test_vowels_count_rec() {
    assert_eq!(count_vowels_rec(""), 0);
    assert_eq!(count_vowels_rec("abEcd"), 2);
    assert_eq!(count_vowels_rec("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("An eagle is outer under"), 10);
}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
    count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1) // 'U'
        ]
    );
    assert_eq!(
        count_vowels_v2("LMAO555 kekW kUsA OwO"),
            [
                ("LMAO555".to_string(), 2),
                ("kekW".to_string(), 1), 
                ("kUsA".to_string(), 2),
                ("OwO".to_string(), 2)
            ]
        );
    
    
}