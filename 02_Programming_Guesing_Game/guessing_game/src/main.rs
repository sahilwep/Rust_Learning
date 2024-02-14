use std::num::ParseIntError;

// function to parse an integer
fn parse_int() -> Result<i32, ParseIntError> {
   // Example of ? where value is unwrapped
   let x: i32 = "12".parse()?; // x = 12

   // Example of ? where error is returned
   let y: i32 = "12a".parse()?;  // returns an Err() immediately

   Ok(x + y) // Doesn't reach this line
}

fn main() {
   let res = parse_int();

   println!("{:?}", res);
}