use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..(n as usize) {
        for j in (i + 1)..(n as usize) {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Створення файлу для запису результату
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Зчитування першого рядка з параметрами n і k
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Зчитування масиву ar
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Виклик функції і запис результату у файл
    let result = divisibleSumPairs(n, k, &ar);
    writeln!(&mut fptr, "{}", result).ok();
}
