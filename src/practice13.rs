fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // Якщо не можливо розподілити рівномірно
    if total % n != 0 {
        return 0; // Або тут можна повернути якесь інше значення
    }

    let average = total / n;

    let mut moves: u32 = 0; // Змінюємо тип на u32

    for &shipment in shipments {
        if shipment > average {
            moves += shipment - average;
        } else {
            moves += average - shipment;
        }
    }

    // Перетворюємо moves з u32 на usize
    moves as usize
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = Vec::new();
    let base = n as u32; // Базовий обсяг

    for _ in 0..n {
        shipments.push(base);
    }

    shipments
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let result1 = count_permutation(&shipments1);
    println!("Результат 1: {}", result1); // Виведе 4

    let shipments2 = vec![9, 3, 7, 2, 9];
    let result2 = count_permutation(&shipments2);
    println!("Результат 2: {}", result2); // Виведе 7
}
