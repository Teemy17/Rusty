fn pack_number_tuples3(l1: &[i32], l2: &[i32], l3: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    let max_len = l1.len().max(l2.len()).max(l3.len());

    for i in 0..max_len {
        let n1 = if i < l1.len() { l1[i] } else { 0 };
        let n2 = if i < l2.len() { l2[i] } else { 0 };
        let n3 = if i < l3.len() { l3[i] } else { 0 };
        result.push((n1, n2, n3));
    }

    return result
}



#[test]
fn test_pack_number_tuples() {
    assert_eq!(pack_number_tuples3(&[1, 2], &[4, 3], &[5]), [(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&[1, 2, 6], &[4, 3], &[5, 7, 8]), [(1, 4, 5), (2, 3, 7), (6, 0, 8)]);
    assert_eq!(pack_number_tuples3(&[], &[], &[]), []);
    assert_eq!(pack_number_tuples3(&[1, 2, 6], &[], &[5, 7, 8]), [(1, 0, 5), (2, 0, 7), (6, 0, 8)]);

}

