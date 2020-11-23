pub fn _move_ownership() {
  let s1 = String::from("New value");
  let _s2 = s1;

  println!("This will not work because ownership of the the heap area controlled by s1 moved to s2: {}", s1);
}

pub fn _clone() {
  let s1 = String::from("New value");
  let _s2 = s1.clone();

  println!("This will work because s1 and s2 own different heap memory: {} {}", s1, _s2);
}

fn takes_ownership(s: String) {
  println!("Ownership from s1 below moved to s: {}", s)
}

pub fn _func_param_takes_ownership() {
  let s1 = String::from("New value");
  takes_ownership(s1);
  println!("This will not work because ownership of the the heap area controlled by s1 moved to s of takes_ownership: {}", s1);
}

fn returns_ownership() -> String {
  let s1 = String::from("New value");
  s1
}

pub fn _func_return_value_moves_ownership() {
  let s = returns_ownership();
  println!("This will  work because ownership of s1 from returns_ownership moves to _s: {}", s);
}

// s borrows the s1 below
fn calculate_length(s: &String) -> usize {
  s.len()
}

pub fn _ref_param() {
  let s1 = String::from("New value");
  let len = calculate_length(&s1);

  println!("The length is {}", len);

  //This will work because we have have created a reference
  // that points to s1 on the stack which in turns points to 
  // the occupied heap area that stores the string 
  println!("{}", s1);
}

// References are immutable by default same as all the other values in Rust.
// If we want to mutate the heap area that the borrowed variable i.e. s1 points to 
// we need to make s1 mutable and use mutable references
fn change(str: &mut String) {
  str.push_str(", world");
}

pub fn _mutable_ref() {
  let mut s1 = String::from("New value");
  change(&mut s1)
}

// But mutable references have one big restriction: you can have 
// only one mutable reference to a particular piece of data in a 
// particular scope. This code will fail:
pub fn _cannot_borrow_twice() {
  let mut s1 = String::from("New value");

  let _s2 = &mut s1;
  let _s3 = &mut s1;

  println!("{}, {}", _s2, _s3);
}
