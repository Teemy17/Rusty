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
