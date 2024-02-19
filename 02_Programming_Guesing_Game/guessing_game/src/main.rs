
fn main(){
   // define enum with multiple variants and data types
   #[derive(Debug)]
   enum Animal {
      Dog(String, f64),
      Cat(String, f64),
   }

   // initialize a mutable enum variant with values
   let mut dog = Animal::Dog(String::from("Benny"), 37.5);

   
   // initialize a non-mutable enum variant with values
   let cat = Animal::Dog(String::from("maya"), 22.4);

   // print enum values before changing : 
   println!("Dog before = {:?}", dog);
   println!("Cat before = {:?}", cat);

   // change the value of mutable enum variant
   dog = Animal::Dog(String::from("Sterling"), 21.3);

   // print enum values after change.
   println!("\n\nDog after = {:?}", dog);
   println!("Cat after = {:?}", cat);
}