pub fn run() {
    let mut hello = String::from("Hello");

    println!("Length: {}", hello.len());

    // push char
    hello.push('W');

    // push string
    hello.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("Worlds"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    assert_eq!(10, s.capacity());

    println!("{}", hello);
}