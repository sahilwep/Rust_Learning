fn main(){
   // define a struct with generic data type
   #[derive(Debug)]
   struct Point<T> {
      x: T,
      y: T,
   }

   // initializing a generic struct with i32 data type
   let int_point = Point {x: 1, y: 2};
   
   // initializing a generic struct with f32 data type
   let float_point = Point {x: 1.2, y: 2.3};

   println!("int_point = {:?}", int_point);
   println!("float_point = {:?}", float_point);
}

