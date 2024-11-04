use std::io::{self, BufRead};


fn miniMaxSum(arr: &[i32]) {
    let total_sum: i32 = arr.iter().sum(); // Сумуємо всі елементи масиву
    let min_value = *arr.iter().min().unwrap(); // Знаходимо мінімальний елемент
    let max_value = *arr.iter().max().unwrap(); // Знаходимо максимальний елемент

    let min_sum = total_sum - max_value; // Мінімальна сума (виключаємо максимальний елемент)
    let max_sum = total_sum - min_value; // Максимальна сума (виключаємо мінімальний елемент)

    println!("{} {}", min_sum, max_sum); // Виводимо результати
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
