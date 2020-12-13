pub fn print() {
    // print to console
    println!("Hello world, that's harshal");

    // basic formatting
    println!("{} is a {}", "harshal", "Coder");

    // positional arguments
    println!("{0} is a {1} and also likes {1}, so go to {0}", "Harshal", "Coder");

    // named arguments
    println!("{name} likes to play {sport}", name="harshal", sport="football");

    // placeholder traits
    println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}