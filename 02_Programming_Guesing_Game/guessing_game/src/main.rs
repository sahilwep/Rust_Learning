fn main(){
   let mut c:Vec<i128>=vec![ 1,2,3,7];
   println!("The value of c is {:?}",c);
   let  d: &mut Vec<i128>= &mut c;
   println!("The value of d is {:?}",d);
   d.push(5);
   d.insert(1,700);
   d.remove(2);
   println!("The value of d after insertion  is {:?}",d);
   println!("The value of c is {:?}",c);

}