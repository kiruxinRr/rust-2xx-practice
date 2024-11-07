fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    // Edge case: If m is greater than the length of s, return 0
    if m as usize > s.len() {
        return 0;
    }

    // Initialize a counter for valid segments
    let mut count = 0;

    // Iterate through each possible starting index
    for i in 0..=s.len() - m as usize {
        // Calculate the sum of the current subarray of length m
        let sum: i32 = s[i..i + m as usize].iter().sum();

        // If the sum matches d, increment the count
        if sum == d {
            count += 1;
        }
    }

    // Return the count of valid segments
    count
}
