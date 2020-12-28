struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

fn create_with_shorthand_notation(email:String, username:String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 0
  }
}

fn create_from_existing(email:String, username:String, user:User) -> User {
  User {
    email,
    username,
    ..user
  }
}

pub fn create_user() {
  let user1 = User {
    email: String::from("ppoliani@gmail.com"),
    username: String::from("ppoliani"),
    active: true,
    sign_in_count: 10
  };

  let mut user2 = User {
    email: String::from("ppoliani@gmail.com"),
    username: String::from("ppoliani"),
    active: true,
    sign_in_count: 10
  };

  user2.username = String::from("ppoliani");

  let user3 = create_with_shorthand_notation(
    String::from("ppoliani@gmail.com"), 
    String::from("ppoliani")
  );

  let user4 = create_from_existing(
    String::from("ppoliani@gmail.com"), 
    String::from("ppoliani"),
    user3
  );
}

// This is a tuple struct. This is useful when we want to make a tuple a different type
// from other typles and naming each field as in regular struct would be to verbose
struct Point(i32, i32);

pub fn tuple_struct() {
  let p1 = Point(0, 0);
}

#[derive(Debug)]
pub struct Rectangle {
  width: u32,
  height: u32
}

fn area(rect:&Rectangle) -> u32 {
  rect.width * rect.height
}


pub fn rectangle_area() {
  let rect = Rectangle { width: 10, height: 20 };
  let sqm = |rect:&Rectangle| -> u32 {area(rect)};

  // using :? tells println! to use an output format called Debug
  // To enable it the Rectangle struct should derive the Debug trait
  println!("The area is for the rectangle {:?} is {}", rect, sqm(&rect))
}
