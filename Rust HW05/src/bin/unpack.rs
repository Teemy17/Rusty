fn unpack_number_tuples(tuples: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for tuple in tuples {
        v1.push(tuple.0);
        v2.push(tuple.1);
    }

    return (v1, v2)
}

fn unpack_number_tuples3(tuples: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();

    for tuple in tuples {
        v1.push(tuple.0);
        v2.push(tuple.1);
        v3.push(tuple.2);
    }

    return (v1, v2, v3)
}

#[test]
fn test_unpack_number_tuples(){
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 2)]), (vec![1], vec![2]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (2, 5), (3, 6)]), (vec![1, 2, 3], vec![4, 5, 6]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]),  (vec![1, 3, 2], vec![4, 2, 1]));
    assert_eq!(unpack_number_tuples(&[(-1, -4), (-2, -5), (-3, -6)]), (vec![-1, -2, -3], vec![-4, -5, -6]));
}

#[test]
fn test_unpack_number_tuples3(){
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(1, 2, 3)]), (vec![1], vec![2], vec![3]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1)]), (vec![1, 2], vec![4, 2], vec![5, 1]));
    assert_eq!(unpack_number_tuples3(&[(-1, -4, -5), (-2, -2, -1)]), (vec![-1, -2], vec![-4, -2], vec![-5, -1]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1), (4, 5, 6)]), (vec![1, 2, 4], vec![4, 2, 5], vec![5, 1, 6]));
}
