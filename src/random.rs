pub struct RandCtx {
    last_random: u128,
    randoms_used: u128,
}

pub fn new(seed: u128) -> RandCtx {
    return RandCtx {
        last_random: seed,
        randoms_used: 0,
    };
}

pub fn generate(ctx: &mut RandCtx) -> f64 {
    ctx.randoms_used += 1;
    let m = 2_u128.pow(27);
    let a = 545643;
    let c = 76785897;
    ctx.last_random = ((a * ctx.last_random) + c) % m;
    return ctx.last_random as f64 / m as f64;
}
