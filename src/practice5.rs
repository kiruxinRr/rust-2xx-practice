const SIZE: usize = 11;

#[test]
fn main() {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if x == 0 || x == SIZE - 1 || y == 0 || y == SIZE - 1 || x == y || x == SIZE - y - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
