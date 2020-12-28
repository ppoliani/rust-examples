fn find_first_word(s:&String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i]
    }
  }

  &s[..]
}

pub fn first_word() {
  let mut s = String::from("The first word of the sentence is The");
  let word = find_first_word(&s);
  
  // this will fail because  clear needs to get a mutable reference to s.
  // However the variable word is an immutable borrower of the initial string
  // so we cannot have both immutable and mutable borrowers
  s.clear(); 
  println!("The first word is {}", word);
}
