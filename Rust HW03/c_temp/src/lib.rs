fn fahr_to_cel_v( v: &mut[i64]) -> Vec<i64>  {
    for n in 0..v.len() {
      v[n] = ((5.0 / 9.0) * (v[n] as f64 - 32.0)) as i64;
     }
     v.to_vec()
}

fn fahr_to_cel_rec( v: &mut[i64]) -> Vec<i64> {
     if !v.is_empty(){
          v[0] = ((5.0 / 9.0) * (v[0] as f64 - 32.0)) as i64;
          fahr_to_cel_rec(&mut v[1..]);
      }
      v.to_vec()
}


#[test]
fn test_fahr_to_cel_v() {
assert_eq!(fahr_to_cel_v(&mut[5, -4, -5, 0, 4, 8, 28, 1, 2, 10]), [-15, -20, -20, -17, -15, -13, -2, -17, -16, -12]);
assert_eq!(fahr_to_cel_v(&mut[10, 20, 30, 40]), [-12, -6, -1, 4]);
assert_eq!(fahr_to_cel_v(&mut[10]), [-12]);
}

#[test]
fn test_fahr_to_cel_rec() {
assert_eq!(fahr_to_cel_rec(&mut[5, -4, -5, 0, 4, 8, 28, 1, 2, 10]), [-15, -20, -20, -17, -15, -13, -2, -17, -16, -12]);
assert_eq!(fahr_to_cel_rec(&mut[10, 20, 30, 40]), [-12, -6, -1, 4]);
assert_eq!(fahr_to_cel_rec(&mut[10]), [-12]);
}