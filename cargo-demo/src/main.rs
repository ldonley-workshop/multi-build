use rand::Rng;

fn main() {
    add_two_numbers(2, 3);
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen();
    println!("Random number: {}", random_number);
}
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(add_two_numbers(2, 3), 5);
}
