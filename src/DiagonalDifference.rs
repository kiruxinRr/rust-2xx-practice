use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    let n = arr.len();
    for i in 0..n {
        primary_diagonal_sum += arr[i][i];            // Головна діагональ
        secondary_diagonal_sum += arr[i][n - i - 1];  // Побічна діагональ
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs() // Повертаємо абсолютну різницю
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    for i in 0..n {
        arr.push(Vec::with_capacity(n));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
