fn main(){

   let text = "Hello, world!";

   let character_option = text.chars().nth(11);

   // using match for Options type
   let character = match character_option {
      None => "empty".to_string(),
      Some(c) => c.to_string()
   };

   println!("Character at index 11 is {}", character);
}
