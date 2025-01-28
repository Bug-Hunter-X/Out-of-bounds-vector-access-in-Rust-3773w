fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    // Use get() to safely access elements
    match vec.get(10) {
        Some(value) => println!("Valid element: {}", value),
        None => println!("Index out of bounds"),
    }
    //Or use if let to make it concise
    if let Some(value) = vec.get(1) {
        println!("Second element: {}", value);
    }
} 