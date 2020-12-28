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


// Add methods to the struct
impl Rectangle {
  // An associated function rather that a method that has access to the
  // the instance. Usually used as a static function associated with the 
  // the struct or as a constructor i.e. String::from
  fn create(width:u32, height:u32) -> Rectangle {
    Rectangle {width, height}
  }

  // Methods can
  // 1. take ownership of self i.e self
  // 2. borrow self immutable i.e. &self
  // 3. borrow self mutably i.e. &mut self
  // Having a method that takes ownership of instance by using just self is rate.
  // This is usually used when the method transforms self into something else and you want to 
  // prevent the caller from using the original instance after the transformation
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn equals(&self, other:&Rectangle) -> bool {
    self.width == other.width
      && self.height == other.height
  }
}

fn area(rect:&Rectangle) -> u32 {
  rect.width * rect.height
}


pub fn rectangle_area() {
  let rect = Rectangle { width: 10, height: 20 };
  let rect2 = Rectangle { width: 20, height: 20 };
  let sqm = |rect:&Rectangle| -> u32 {area(rect)};

  // using :? tells println! to use an output format called Debug
  // To enable it the Rectangle struct should derive the Debug trait
  println!("The area for the rectangle {:?} is {}", rect, sqm(&rect));

  // use a method on the struct instead of an external function
  println!("The area using a method for the rectangle {:?} is {}", rect, rect.area());

  println!("Is rect {:?} equal to {:?} -> {}", rect, rect2, rect.equals(&rect2));

  println!("A associated function creating new rectangle {:?}", Rectangle::create(10, 30));
}
