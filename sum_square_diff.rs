fn main() {
    let mut sqrt_sum = 0;
    let mut num_sum = 0;
    for x in 0..=100 {
        sqrt_sum += u32::pow(x, 2);
        num_sum += x;
    }

    let total = u32::pow(sqrt_sum, 2) - num_sum;

    println!("total: {}", total)
}
