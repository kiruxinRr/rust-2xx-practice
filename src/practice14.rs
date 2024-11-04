#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

// Функція для знаходження зайнятої площі прямокутників
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points = std::collections::HashSet::new();

    for rect in xs {
        // Знайдемо координати
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        // Додаємо всі точки прямокутника до множини
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                occupied_points.insert((x, y));
            }
        }
    }

    occupied_points.len() as i32
}

// Тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Тест функції
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
