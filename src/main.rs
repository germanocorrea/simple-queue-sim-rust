mod random;

fn main() {
    println!("Testing the random number generator!");
    let mut ctx = random::new(42);
    for i in 0..5 {
        let r = random::generate(&mut ctx);
        println!("Random {}: {} ", i, r);
    }
}
