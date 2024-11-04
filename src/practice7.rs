#[test]
fn main() {
    let triangles = 5; // Змініть на потрібну кількість трикутників
    draw_christmas_tree(triangles);
}

fn draw_christmas_tree(triangles: usize) {
    for i in 1..=triangles {
        let base_width = 1 + (i - 1) * 2;
        for j in 0..i {
            let width = base_width + j * 2;
            let padding = " ".repeat(triangles + triangles - j - i);
            let stars = "*".repeat(width);
            println!("{}{}", padding, stars);
        }
    }

    // Малюємо стовбур
    let trunk_padding = " ".repeat(triangles + triangles - 2);
    for _ in 0..3 {
        println!("{}*", trunk_padding);
    }
}
