fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut reversed = 0;

    let mut num = x;
    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    original == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}
