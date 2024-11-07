use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    // Знаходимо максимальну висоту
    let max_height = *candles.iter().max().unwrap();

    // Рахуємо, скільки свічок мають максимальну висоту
    candles.iter().filter(|&&height| height == max_height).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Створюємо файл для запису результату
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Зчитуємо кількість свічок
    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Зчитуємо висоти свічок і перетворюємо їх у вектор цілих чисел
    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Викликаємо функцію і отримуємо результат
    let result = birthdayCakeCandles(&candles);

    // Записуємо результат у файл
    writeln!(&mut fptr, "{}", result).ok();
}
