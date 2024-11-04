use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }

    (min_sum, min_indices.0, min_indices.1)
}

fn print_vector(data: &[i32], min_sum: i32, index1: usize, index2: usize) {
    let index_string: String = (0..data.len())
        .map(|i| format!("{:<2}. ", i))
        .collect();
    let data_string: String = data.iter()
        .map(|num| format!("{:<2}", num))
        .collect();

    let underline = " ".repeat(index_string.len() - (index1 + 1) * 3) + "\\__ __/";

    println!("indexes: {}", index_string);
    println!("data:   [{}]", data_string);
    println!("indexes:{}", underline);
    println!("min adjacent sum={}+{}={} at indexes:{},{}",
             data[index1], data[index2], min_sum, index1, index2);
}

fn main() {
    let random_vector = gen_random_vector(20);
    let (min_sum, index1, index2) = min_adjacent_sum(&random_vector);
    print_vector(&random_vector, min_sum, index1, index2);
}
