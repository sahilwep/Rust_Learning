macro_rules! match_eqn {
    ($a: expr, $b: expr) => {
        {
            let mut res;
            if $a == $b {
                res = 1;
            } else {
                res = 0;
            }
            res
        };
    }
}

fn main(){
    let a: i32 = 3;
    let b: i32 = 4;

    let v = match_eqn!(a, a);

    println!("res = {v} ");

}