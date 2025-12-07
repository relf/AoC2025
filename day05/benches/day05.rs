fn main() {
    divan::main();
}

#[divan::bench]
fn day05_part1_bench() {
    day05::process_part1(include_str!("../input.txt"));
}

#[divan::bench]
fn day05_part2_bench() {
    day05::process_part2(include_str!("../input.txt"));
}
