fn main() {

    let numbers = vec![2, 40, 34, 90, -59, -200, 45, 100];

    let largest_number = get_largest_number(&numbers);

    println!("The largest number is {}", largest_number);
    println!("Numbers list is {:?}", numbers);
    
}

fn get_largest_number(numbers: &[i32]) -> i32 {

    let mut largest_number = numbers[0];

    for &number in numbers {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}
