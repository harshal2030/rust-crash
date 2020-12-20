use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Re-assign value
  numbers[0] = 20;

  // Add on to vectors
  numbers.push(5);
  numbers.push(6);

  // Remove last element of vector
  numbers.pop();

  println!("Single value: {}", numbers[0]);

  // Get array length
  println!("Vector length: {}", numbers.len());

  // Arrays are allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("{}", x);
  }

  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}