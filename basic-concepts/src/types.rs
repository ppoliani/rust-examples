fn _integers() {
  // Unsigned integers; the corresponding signed start with i i.e. i8, i16 etc
  let _x: u8 = 10;
  let _x: u16 = 257;
  let _x: u32 = 4_294_967_295;
  let _x: u64 = 18_446_744_073_709_551_615;
  let _x: u128 = 18_446_744_073_709_551_617;

  // hex integer literal
  let _x: u8 = 0xff;

  // binary integer literal
  let _x: u8 = 0b1111_0000;

  // byte integer literal
  let _x: u8 = b'A';
}

fn _float() {
  let _x: f32 = 12.00;
  let _x: f64 = 12.12345;
} 

fn _numerical_operations() {
  let _sum = 5 + 5;
  let _diff = 90.5 - 20.4;
  let _prod = 10 * 20;
  let _div = 100 / 10;
  let _remainder = 100 % 10;

}

fn _bool() {
  let _is_true = true;
  let _is_true: bool = false;
}

fn _char() {
  // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, 
  // which means it can represent a lot more than just ASCII.
  let _char: char = 'a';
  let _char = 'b';
  let _emoji = '😻';
}

fn _tuple() {
  // A tuple is a general way of grouping together a number of 
  // values with a variety of types into one compound type.
  let _tuple = (10, 10.0, 'a');
  let _tuple: (u8, f32, char) = (10, 10.0, 'a');

  // read value via destructuring
  let (_x, _y, _z) = _tuple;

  // index access
  let _x = _tuple.0;
  let _x = _tuple.1;
  let _x = _tuple.2;
  
}

fn _array() {
  //  Arrays in Rust are different from arrays in some other 
  // languages because arrays in Rust have a fixed length, like tuples.
  let _arr = [1, 2, 3, 4];

  // Here, i32 is the type of each element. After the semicolon, 
  // the number 5 indicates the array contains five elements.
  let _arr: [i32; 4] = [1, 2, 3, 4];

  // access via index
  let _x = _arr[0];
}
