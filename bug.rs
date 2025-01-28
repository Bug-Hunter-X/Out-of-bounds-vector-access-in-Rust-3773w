fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    // Accessing an out-of-bounds index will cause a panic
    let invalid = vec[10];
    println!("Invalid element: {}", invalid);
}