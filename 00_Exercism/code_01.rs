use std::convert::TryInto;

// Better Solution using : pow() method.
pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = num.to_string().len() as u32;

    let mut sum: u64 = 0;
    let mut n = num as u64;
    while n > 0 {
        sum += (n % 10).pow(digit_count);
        n /= 10;
    }
    sum == num as u64
}



// Bruteforce solution
pub fn is_armstrong_number_1(num: u32) -> bool {
    let mut solu: u64 = 0;
    let mut temp: u64 = num.into();    // used to compute 
    let digit = num.to_string();    // used to convert integer into string
    let data = digit.chars().count();   // use to count the string.

    if data == 1 {
        return true;
    } else {
        while temp > 0 { 
            let ld: u64 = (temp % 10).try_into().unwrap();    // attempted conversion into self
            solu += calc_power(ld, data);
            temp = temp / 10;
        }
    }
    u64::from(num) == solu    // convert num from u32 to u64
}

pub fn calc_power(ld: u64, data: usize) -> u64 {
    let mut solu: u64 = 1;
    for _ in 0..data {
        solu = solu*ld;
    }
    solu
}

fn main(){
    let num: u32 = 153;
    println!("armstrong number {}", is_armstrong_number(num) );
}
