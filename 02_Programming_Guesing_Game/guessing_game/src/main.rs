fn main(){
   let num = 12;  // num comes into scope

   // ownership of num copied into the function
   print_num(num);

   // num variable can be used here
   println!("Fruit = {}", num);
}

fn print_num(num: i8) {
   println!("num is  = {}", num);
} // num goes out of scope