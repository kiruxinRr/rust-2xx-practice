use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8); // Split time from period
    let mut hour: u32 = time[..2].parse().unwrap(); // Parse hour

    if period == "PM" && hour != 12 {
        hour += 12; // Convert PM hour (except 12 PM) to 24-hour format
    } else if period == "AM" && hour == 12 {
        hour = 0; // Convert 12 AM to 00
    }

    format!("{:02}:{:02}:{:02}", hour, &time[3..5], &time[6..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
