fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s; // Повертаємо оригінальний рядок, якщо він порожній
    }

    // Нормалізуємо n, щоб бути в межах довжини рядка
    let n = ((n % len) + len) % len; // Обробляємо від'ємне n коректно

    let (first_part, second_part) = s.split_at((len - n) as usize); // Розділяємо рядок
    format!("{}{}", second_part, first_part) // Об'єднуємо дві частини
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), exp.to_string());
        });
    }
}
