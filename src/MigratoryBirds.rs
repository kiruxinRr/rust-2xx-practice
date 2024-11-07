use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut bird_counts = HashMap::new();

    // Count occurrences of each bird type
    for &bird in arr {
        *bird_counts.entry(bird).or_insert(0) += 1;
    }

    // Find the bird type with the highest frequency, choosing the smallest ID in case of a tie
    bird_counts.iter()
        .max_by_key(|&(bird, &count)| (count, -bird)) // use -bird to prioritize smaller IDs in case of tie
        .map(|(&bird, _)| bird)
        .unwrap_or(0) // default to 0 if no birds are found
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
