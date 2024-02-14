
// Function to find a user by their username which returns as Option enum
fn get_user(username: &str) -> Option<&str> {
   if username.is_empty() {
      return None;
   }
   return Some(username);
}

fn main(){
      // use of unwrap method of get the result of Option enum from get_user function
      let result = get_user("Sahil").unwrap();

      // print the result 
      println!("User = {:?}", result);
}
