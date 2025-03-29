use ndarray::{array, Array1, Array2};
use rand::Rng;
use rand::rng; // Corrected from `thread_rng`

fn generate_values() -> (i32, i32) {
    let mut rng = rng(); // `thread_rng()` is now `rng()`
    let x: i32 = rng.random_range(1..=1000); // `gen_range()` is now `random_range()`
    let y: i32 = rng.random_range(1..=1000);
    (x, y)
}

fn main() {
    let l: Array2<i32> = array![
        [0, 0, 3, 0, 0, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 5, 0, 0, 0]
    ];

    let r: Array2<i32> = array![
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0, 0]
    ];

    let o: Array2<i32> = array![
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1],
        [-3, 1, 1, 2, 0, -1]
    ];

    let (x, y) = generate_values();
    println!("Generated values: x = {}, y = {}", x, y);

    let v1 = 3 * x * x;
    let v2 = v1 * y;
    let out = v2 + 5 * x * y - x - 2 * y + 3;
    let w: Array1<i32> = array![1, out, x, y, v1, v2];

    println!("Computed values: w = {:?}", w);

    let left = o.dot(&w);
    let right = l.dot(&w) * r.dot(&w);

    println!("Left side: {:?}", left);
    println!("Right side: {:?}", right);

    assert_eq!(left, right, "Result contains an inequality");

    println!("Verification successful!");
}
