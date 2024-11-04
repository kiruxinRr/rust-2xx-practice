use std::io::{self, BufRead};

fn plusMinus(arr: &[i32]) {
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;
    let total = arr.len() as f64;

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    // Виведення з точністю до 6 знаків після коми
    println!("{:.6}", positive_count as f64 / total);
    println!("{:.6}", negative_count as f64 / total);
    println!("{:.6}", zero_count as f64 / total);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}