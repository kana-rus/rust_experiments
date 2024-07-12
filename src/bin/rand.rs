fn main() {
    use rand::prelude::*;
    let a: [usize; 10] = {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(314159265358979);
        std::array::from_fn(|_| rng.next_u64() as usize)
    };
    let b: [usize; 10] = {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(314159265358979);
        std::array::from_fn(|_| rng.next_u64() as usize)
    };
    let c: [usize; 10] = {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(314159265358979);
        std::array::from_fn(|_| rng.next_u64() as usize)
    };
    assert_ne!(a, b);
}
