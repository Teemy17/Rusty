fn min_max_avg(v: &[i32]) -> (i32, i32, f64) {
    let (min, max) = min_max_values(v);
    let avg = average(v);
    return (min, max, avg)
}       

fn min_max_values(v: &[i32]) -> (i32, i32) {
    let mut iter = v.iter();
    if let Some(mut min) = iter.next() {
        let mut max = min;
        while let Some(x) = iter.next() {
            if x < min {
                min = x;
            }
            if max < x {
                max = x;
            }
        }
        (*min, *max)
    } else {
        (0, 0)
    }
}

fn average(v: &[i32]) -> f64 {
    if v.is_empty() {
        return 0.0;
    } else {
        let sum: i32 = v.iter().sum();
        sum as f64 / v.len() as f64
    }
}

#[test]
fn tests_min_max_avg() {
    assert_eq!(min_max_avg(&[1, 2, 3, 4, 5]), (1, 5, 3.0));
    assert_eq!(min_max_avg(&[]), (0, 0, 0.0));
    assert_eq!(min_max_avg(&[43, 52, 23, 70, 21]), (21, 70, 41.8))
    
}