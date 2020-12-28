fn find_first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i]
    }
  }

  &s[..]
}

pub fn first_word(s:&String) {
  let word = find_first_word(s);
  println!("The first word is {}", word);
}
