use std::io::{self, BufRead, Write};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade >= 38 && grade % 5 >= 3 {
                grade + (5 - grade % 5)
            } else {
                grade
            }
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Parse the number of grades
    let grades_count: usize = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Expected an integer for number of grades");

    // Collect grades into a vector
    let mut grades: Vec<i32> = Vec::with_capacity(grades_count);
    for _ in 0..grades_count {
        let grade: i32 = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse()
            .expect("Expected an integer grade");
        grades.push(grade);
    }

    // Process the grades and print results
    let result = grading_students(&grades);
    for rounded_grade in result {
        println!("{}", rounded_grade);
    }
}
