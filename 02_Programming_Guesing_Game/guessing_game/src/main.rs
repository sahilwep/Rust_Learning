fn main(){
   let numbers: Vec<i32> = vec![1, 2, 3];

   // using the map iterator adapter
   let even_number: Vec<i32> = numbers.iter().map(|i| i*2).collect();

   println!("Numbers = {:?}", numbers);
   println!("even_numbers = {:?}", even_number);
}
