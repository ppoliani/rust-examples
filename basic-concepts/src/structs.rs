struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
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
}
