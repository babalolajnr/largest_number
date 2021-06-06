fn main() {

    let numbers = vec![2, 40, 34, 90, -59, -200, 45, 100];
    let char_list = vec!['y', 'm', 'a', 'q', 'z'];


    let largest_number = largest(&numbers);
    
    println!("The largest number is {}", largest_number);
    println!("Numbers list is {:?}", numbers);
    
    let largest_number = largest(&char_list);
    
    println!("The largest char is {}", largest_number);
    println!("Char list is {:?}", char_list);
    
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest_number = list[0];

    for &number in list {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}
