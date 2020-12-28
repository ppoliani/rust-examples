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

