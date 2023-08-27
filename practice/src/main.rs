fn main() {
    let v = [2, 1, 3];
    let min = min_values(&v);
    println!("{min}");

    let max = max_values(&v);
    println!("{max}")
}

fn min_values(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    if let Some(mut min) = iter.next() {
        while let Some(x) = iter.next() {
            if x < min {
                min = x;
            }
        }
        *min
    }
    else {
        0
    }
}

fn max_values(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    if let Some(mut max) = iter.next() {
        while let Some(x) = iter.next() {
            if x > max {
                max = x;
            }
        }
         *max
      }
    else {
        0
    }
}
        
    

