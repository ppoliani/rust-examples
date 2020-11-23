fn main() {
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

  // Floating point
  let _x: f32 = 12.00;
  let _x: f64 = 12.12345;
}
