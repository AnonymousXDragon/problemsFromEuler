use num_bigint::BigUint;

fn main(){
    
    let base = BigUint::from(2u32);
    let result = base.pow(1000);
    let result_as_str = res.to_string();
    let mut output = 0;
    
    for ch in result_as_str.chars() {
        output += ch.to_digit(10).expect("unable to unwrap value");
    };
    
    println!("{}",output)
}