use std::collections::HashSet;

fn main(){
   let hashset1 = HashSet::from([2, 7, 8]);
   let hashset2 = HashSet::from([1, 2, 7, 9]);

   // Intersection of HashSets
   let result: HashSet<_> = hashset1.difference(&hashset2).collect();

   println!("hashset1 = {:?}", hashset1);
   println!("hashset2 = {:?}", hashset2);
   println!("Symmetric Difference = {:?}", result);
}