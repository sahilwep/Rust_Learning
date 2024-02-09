fn main(){

    let mut v: Vec<u8> = Vec::new();

    v.push(10);
    v.push(12);
    v.push(13);
    v.push(14);

    for i in 0..4 {
        println!("Value is : {}", v[i]);
    }

}