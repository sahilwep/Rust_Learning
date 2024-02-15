fn main(){
   let mut str = String::from("Sahil");

   // mutable reference 1
   let ref1 = &mut str;
   
   // error 
   // mutable reference 2
   // let ref2 = &mut str;

   println!("{}, {} ", ref1, ref2);
}