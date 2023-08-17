fn extract_quoted_words(input: &str) -> Vec<&str> {
    let mut result = Vec::new();

    for word in input.split_whitespace() {
        match word {
            "" => {}
            word 
            if word.starts_with('*') && word.ends_with('*') => {
                result.push(&word[1..word.len() - 1]);
            }
            _ => {}
        }
    }

    return result
}

fn extract_quoted_words_rec(input: &str) -> Vec<String> {
    fn rec<'a>(mut words: std::str::SplitWhitespace<'a>, mut result: Vec<String>) -> Vec<String> {
        if let Some(word) = words.next() {
            if word.starts_with('*') && word.ends_with('*') {
                result.push(word[1..word.len() - 1].to_string());
            }
            rec(words, result)
        } else {
            return result
        }
    }
    let words = input.split_whitespace();
    rec(words, Vec::new())
}


#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<&str>::new());
    assert_eq!(
        extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        vec!["", "C++", "Python"]
    );
}

#[test]
fn test_extract_quoted_words_rec() {
    assert_eq!(extract_quoted_words_rec(""), Vec::<&str>::new());
    assert_eq!(
        extract_quoted_words_rec("C ** *C++* *Java *Python* Rust*"),
        vec!["", "C++", "Python"]
    );
}