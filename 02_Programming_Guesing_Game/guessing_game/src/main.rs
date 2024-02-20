// use derive keyword to generate implementation of Copy and Clone
#[derive(Copy, Clone)]
struct MyStruct {
    value: i32,
}

fn main(){
    let x = MyStruct{value: 20};
    let y = x;
    
    println!("x: {:?}", x.value);
    println!("x: {:?}", y.value);
}