// fn move_ownership() {
//   let s1 = String::from("New value");
//   let _s2 = s1;

//   println!("This will not work because ownership of the the heap area controlled by s1 moved to s2: {}", s1);
// }

// fn clone() {
//   let s1 = String::from("New value");
//   let _s2 = s1.clone();

//   println!("This will work because s1 and s2 own different heap memory: {} {}", s1, _s2);
// }

// fn takes_ownership(s: String) {
//   println!("Ownership from s1 below moved to s: {}", s)
// }

// fn func_param_takes_ownership() {
//   let s1 = String::from("New value");
//   takes_ownership(s1);
//   println!("This will not work because ownership of the the heap area controlled by s1 moved to s of takes_ownership: {}", s1);
// }

// fn returns_ownership() -> String {
//   let s1 = String::from("New value");
//   s1
// }

// fn func_return_value_moves_ownership() {
//   let s = returns_ownership();
//   println!("This will  work because ownership of s1 from returns_ownership moves to _s: {}", s);
// }

// // s borrows the s1 below
// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// fn ref_param() {
//   let s1 = String::from("New value");
//   let len = calculate_length(&s1);

//   println!("The length is {}", len);

//   // This will work because we have have created a reference
//   // that points to s1 on the stack which in turns points to 
//   // the occupied heap area that stores the string 
//   println!("{}", s1);
// }

// // References are immutable by default same as all the other values in Rust.
// // If we want to mutate the heap area that the borrowed variable i.e. s1 points to 
// // we need to make s1 mutable and use mutable references
// fn change(str: &mut String) {
//   str.push_str(", world");
// }

// fn _mutable_ref() {
//   let mut s1 = String::from("New value");
//   change(&mut s1)
// }

// // But mutable references have one big restriction: you can have 
// // only one mutable reference to a particular piece of data in a 
// // particular scope. This code will fail:
// fn cannot_borrow_twice_within_the_same_scope() {
//   let mut s1 = String::from("New value");

//   let _s2 = &mut s1;
//   let _s3 = &mut s1;

//   println!("{}, {}", _s2, _s3);

//   let mut s = String::from("hello");

//   {
//       let _r1 = &mut s;
//   } // r1 goes out of scope here, so we can make a new reference with no problems.

//   let _r2 = &mut s;
// }

// fn borrow_as_immutable_and_then_as_mutable() {
//   let mut s1 = String::from("New value");

//   let _s2 = &s1;
//   let _s3 = &s1;
//   let _s4 = &mut s1;

//   println!("{}, {}, and {}", _s2, _s3, _s4);
// }

// // Note that a reference’s scope starts from where it is introduced 
// // and continues through the last time that reference is used.
// fn reference_scope() {
//   let mut s1 = String::from("New value");

//   let _s2 = &s1;
//   let _s3 = &s1;

//   println!("{}, {}", _s2, _s3);

//   // no problem because _s2 and _s3 references went out of scope when they were last used above
//   let _s4 = &mut s1;
//   println!("{}", _s4);

//   // BUT if we use it again here then they're still in scope so the previous code will fail
//   println!("{}, {}", _s2, _s3);
// }

// fn dangle() {
//   let s = _dangle_ref();
// }

// // Because is created in the scope of the dangle_ref function
// // when the function is executed it goes out of scope and thus it
// // will be dropped. However, there is a reference that we return which 
// // now point to an empty memory location. Rust will prevent this situation
// fn dangle_ref() -> &String {
//   let s = String::from("Some string");

//   &s
// }
