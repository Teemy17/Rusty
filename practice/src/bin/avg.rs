fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let average = calculate_average(&numbers);
    println!("Average: {:.2}", average);
}

fn calculate_average(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as f64;
    return sum as f64 / count
}
