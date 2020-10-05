use fuzz_targets::common::SeedGen;

pub fn gen_vec_u8(gen: &mut SeedGen) -> Vec<u8> {
    let data = gen.generate(proptest::arbitrary::any::<Vec<u8>>());
    data
}
pub fn gen_str(gen: &mut SeedGen) -> String {
    let data = gen.generate(proptest::arbitrary::any::<String>());
    data
}
pub fn gen_usize(gen: &mut SeedGen) -> usize {
    let data = gen.generate(proptest::arbitrary::any::<usize>());
    data
}
pub fn gen_u8(gen: &mut SeedGen) -> u8 {
    let data = gen.generate(proptest::arbitrary::any::<u8>());
    data
}
pub fn gen_i8(gen: &mut SeedGen) -> i8 {
    let data = gen.generate(proptest::arbitrary::any::<i8>());
    data
}
