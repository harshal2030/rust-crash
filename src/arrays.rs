use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assgin value
  numbers[0] = 20;

  println!("Single value: {}", numbers[0]);

  // Get array length
  println!("Array length: {}", numbers.len());

  // Arrays are allocated
  println!("Array occpuies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}