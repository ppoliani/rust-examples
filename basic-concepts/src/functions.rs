pub fn _func() {
  println!("This is function.");
}

// In function signatures, you must declare the type of each parameter
pub fn _func_with_params(x: i8, y: f32) {
  println!("The value of x is: {} and of y: {}", x, y);
}

pub fn _func_that_returns(x: i8) -> i8 {
  x * 10
}

pub fn _closures() {
  // similar to arrow or lambda or inline function definition
  let _closure = |i: i32| -> i32 { i + 10 };
  let _closure_inferred: fn(i32) -> i32 = |i| i + 10 ;
}

pub fn _func_type() -> fn() -> () {
  // return a new function that accepts no parameters and executed a 
  // statement i.e. doesn't return anything (empty tuple () in rust)
  || -> () { println!("Hey there") }
}
