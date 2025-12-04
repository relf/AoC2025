fn main() {
    divan::main();
}

#[divan::bench]
fn day01_part1_bench() {
    day01::process_part1(include_str!("../input.txt"));
}

#[divan::bench]
fn day01_part2_bench() {
    day01::process_part2(include_str!("../input.txt"));
}
