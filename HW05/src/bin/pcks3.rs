fn pack_number_tuples_s3(tuple1: &[i32], tuple2: &[i32], tuple3: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut iter1 = tuple1.iter();
    let mut iter2 = tuple2.iter();
    let mut iter3 = tuple3.iter();
    let mut result = Vec::new();

    while let (Some(&n1), Some(&n2), Some(&n3)) = (iter1.next(), iter2.next(), iter3.next()) {
        result.push((n1, n2, n3));
    }
   
    return result

}

#[test]
fn test_pack_number_tuples_s3() {
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[5]), [(1, 4, 5)]);
    assert_eq!(pack_number_tuples_s3(&[1, 2, 6], &[4, 3], &[5, 7, 8]), [(1, 4, 5), (2, 3, 7)]);
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), []);
}