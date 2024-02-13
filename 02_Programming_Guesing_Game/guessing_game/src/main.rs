use std::fs::File;

fn main(){
   let data_result = File::open("data.txt");

   // using match for Result type
   let data_file = match data_result {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the data file : {:?}", error),
   };

   println!("Data file : {:?}", data_file);
}