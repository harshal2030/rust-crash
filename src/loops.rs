pub fn run () {
  let mut count = 0;

  // Infinite loop if we remove that if and break statement
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }

  let mut fizz_buzz_count = 0;

  // While loop (Fizz Buzz)
  while fizz_buzz_count <= 100 {
    if fizz_buzz_count % 15 == 0 {
      println!("fizzbuzz");
    } else if fizz_buzz_count % 3 == 0 {
      println!("fizz");
    } else if fizz_buzz_count % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", fizz_buzz_count);
    }

    // Increment count
    fizz_buzz_count += 1;
  }

  // FizzBuzz using for loop
  for x in 0..100 {
    if x % 15 == 0 {
      println!("fizzbuzz");
    } else if x % 3 == 0 {
      println!("fizz");
    } else if x % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", x);
    }
  }
}