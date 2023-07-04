use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let numbers = vec![10, 2, 3, 4, 5, 9, 4];

    let median = median(&numbers);

    println!("The median is {}", median);

    let mode = mode(&numbers);

    println!("The mode is {}", mode);
}

fn median(num: &Vec<i32>) -> f64 {
    let mut numbers = num.clone();
    numbers.sort();
    if numbers.len() % 2 == 0 {
        let mid = numbers.len() / 2;
        let a = numbers[mid];
        let b = numbers[mid - 1];
        (a + b) as f64 / 2.0
    } else {
        let mid = numbers.len() / 2;
        numbers[mid] as f64
    }
}

fn mode(num: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in num {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (key, value) in map.iter() {
        if *value > max {
            max = *value;
            mode = **key
        }
    }

    // use map directly will move map
    // map.insert(&4, 5);

    mode
}
