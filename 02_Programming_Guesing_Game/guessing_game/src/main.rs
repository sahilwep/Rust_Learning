fn main(){
    // creating of mutable vector 
    let mut v: Vec<u32> = vec![29, 23, 7, 23];

    v.push(12);
    v.push(92);

    println!("Original Vec = {:?}", v);
    
    // removing value from vector by passing index.
    v.remove(2);

    println!("Original Vec = {:?}", v);
}