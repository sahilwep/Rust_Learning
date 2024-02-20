// A macro which uses repetitions
macro_rules! repeat_print {
   // match rule which matches multiple expression in an argument
   ($($x: expr), *) => {
      $(
         println!("{}", $x);
      )*
   };
}


fn main(){
   // call the macro with multiple arguments
   repeat_print!(1, 2, 3, 4, 5);
}