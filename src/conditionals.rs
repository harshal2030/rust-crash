pub fn run() {
  let age: u8 = 17;
  let check_id: bool = true;
  let knows_age_of_person = true;


  // If/else
  if age > 21 && check_id || knows_age_of_person {
    println!("What would you like to drink?");
  } else if age < 21 && check_id {
    println!("Sorry you have to leave");
  } else {
    println!("I'll need to see your id");
  }

  // Shorthand If
  let is_of_age = if age >= 21 {true} else {false};
  println!("Is of age: {}", is_of_age);
}