// nested module
pub mod player {
   pub mod sprite {
      pub fn create() {
         println!("Called player::sprite::create");
      }
   }
}

// bring the create function into scope
use player::sprite::create;

fn main(){
   // call public function directly
   create();
}