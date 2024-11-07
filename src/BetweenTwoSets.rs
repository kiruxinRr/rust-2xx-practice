fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i32, y: i32) -> i32 {
    (x * y) / gcd(x, y)
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().fold(1, |acc, &num| lcm(acc, num));
    let gcd_b = b.iter().fold(b[0], |acc, &num| gcd(acc, num));

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}
