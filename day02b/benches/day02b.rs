fn main() {
    divan::main();
}

#[divan::bench]
fn day02b_bench() -> u64 {
    day02b::run(include_str!("../input.txt")) as u64
}
