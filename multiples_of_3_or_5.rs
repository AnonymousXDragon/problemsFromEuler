fn main() {
    let mut res = Vec::new();
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            res.push(x);
        }
    }

    let total: u32 = res.iter().sum();
    println!("{}", total)
}
