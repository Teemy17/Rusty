fn double_numbers(numbers: &mut Vec<i32>) {
    for num in numbers.iter_mut() {
        *num *= 2;
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    println!("Original numbers: {:?}", numbers);
    
    double_numbers(&mut numbers);
    
    println!("Doubled numbers: {:?}", numbers);
}

fn doubles_recursive(v: &mut [i64]) -> Vec<i64> {
    if !v.is_empty() {
        v[0] *= 2;
        doubles_recursive(&mut v[1..]);
    }
    v.to_vec()
}
