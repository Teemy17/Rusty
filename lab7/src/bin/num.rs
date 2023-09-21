use rand::Rng;

fn filter_numbers(v: &[f32]) -> Vec<f32> {
    let mut iter = v.iter();
    let mut result = Vec::new();

    if v.is_empty() {
        return result
    }

    while let Some(n) = iter.next() {
        if n >= &-1.0 && n <= &1.0 {
            result.push(*n)
        }
    }
        return result
}


fn gen_number<R: Rng>(rng: &mut R, n: i64) -> Vec<f64> {
    let mut result = Vec::new();
    for i in 0..n {
        let num = rng.gen_range(-10.0 ..=10.0);
        result.push(num)
    }
    return result
}

#[test]
fn test_filter_numbers() {
    assert_eq!(filter_numbers(&[]), ([]));
    assert_eq!(filter_numbers(&[2.0, 0.4, 5.0, -1.0, 0.8]), ([0.4, -1.0, 0.8]));
}

#[test]
fn test_gen_number() {
    let mut rng = rand::thread_rng();

    let generated_empty = gen_number(&mut rng, 0);
    assert_eq!(generated_empty.len(), 0);

    let generated_positive = gen_number(&mut rng, 5);
    assert_eq!(generated_positive.len(), 5);
    for &num in &generated_positive {
        assert!(num >= -10.0 && num <= 10.0);
    }

    let generated_negative = gen_number(&mut rng, -5);
    assert_eq!(generated_negative.len(), 0);
}