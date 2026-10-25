mod random;

fn main() {
    println!("Testing the random number generator!");
    let mut ctx = random::RandomGenerator::new(42);
    for i in 0..5 {
        let r = ctx.generate();
        println!("Random {}: {} ", i, r);
    }
}
