use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// Функція `simple_array_sum` обчислює суму чисел у масиві.
fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Створення файлу для запису результату
    let output_path = env::var("OUTPUT_PATH").expect("Задайте змінну середовища OUTPUT_PATH");
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл");

    // Зчитування кількості елементів масиву
    let _ar_count = stdin_iterator.next()
        .expect("Не вдалося зчитати кількість елементів")
        .expect("Зчитаний рядок порожній")
        .trim()
        .parse::<i32>()
        .expect("Не вдалося перетворити на ціле число");

    // Зчитування масиву чисел
    let ar: Vec<i32> = stdin_iterator.next()
        .expect("Не вдалося зчитати масив")
        .expect("Зчитаний рядок порожній")
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Не вдалося перетворити на ціле число"))
        .collect();

    // Виклик функції для обчислення суми
    let result = simple_array_sum(&ar);

    // Запис результату у файл
    writeln!(&mut fptr, "{}", result).expect("Не вдалося записати у файл");
}
