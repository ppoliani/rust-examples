pub fn _if() {
  let num = 10;

  if num < 20 {
    println!("Num smaller than 20");
  } else if num < 10 {
    println!("Num greater than 10");
  } else {
    println!("None of the above");
  }
}

// Because if is an expression, we can use it on the right side of a let statement,
pub fn _if_as_expression() {
  let cond = 20;
  let _number = if cond < 10 {5} else {10};

  println!("The value is {}", _number);
}

pub fn _loop() {
  let mut x = 0;

  // infinite loop
  loop {
    println!("again!");

    if x > 10 { break; }
    x += 1;
  }
}

pub fn _loop_as_expression() {
  let mut counter: i8 = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("Loop expression evaluated to: {}", result);
}

pub fn _while_loop() {
  let mut num: i8 = 0;

  while num < 10 {
    println!("{}!", num);

    num += 1;
  }
}

pub fn _iterate_array_unsafe() {
  let a = [1, 2, 3, 4, 5];
  let mut i = 0;

  // But this approach is error prone; we could cause the program to panic 
  // if the index length is incorrect. It’s also slow, because the compiler 
  // adds runtime code to perform the conditional check on every element on every iteration through the loop.
  while i < 5 {
    println!("{} -> {}", i, a[i]);
    i += 1;
  }
}

pub fn _iterate_array_safe() {
  let a = [1, 2, 3, 4, 5];

  for element in a.iter() {
    println!("the value is: {}", element)
  }
}


pub fn _iterate_range() {
  for num in (1..4).rev() {
    println!("{}!", num);
  }
}
