fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; // based on data type. If add vec!, the list becomes vector. If only [], it's a list.
    let sum = sum_list(&numbers);
    println!("Sum: {}", sum);
}

fn sum_list(numbers: &Vec<i32>) -> i32 { //based on data type. If lists you can use &[i32] instead.
    let sum = numbers.iter().sum();
    return sum
}
