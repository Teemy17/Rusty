fn cal_partial_sum(n: &[i32]) -> Vec<i32> {
    let mut partial_sum = Vec::new();
    let mut sum = 0;

    for &num in n {
        sum += num;
        partial_sum.push(sum);
    }
    return partial_sum
}


#[test]
fn test_partial_sum() {
    assert_eq!(cal_partial_sum(&[2, 11, 3, 4, 7]), vec![2, 13, 16, 20, 27]);
    assert_eq!(cal_partial_sum(&[5, 3, 4, 56, 43, 12]), vec![5, 8, 12, 68, 111, 123]);
    assert_eq!(cal_partial_sum(&[33, 3, 0, 2, 3]), vec![33, 36, 36, 38, 41]);
}