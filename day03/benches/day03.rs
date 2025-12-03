fn main() {
    divan::main();
}

#[divan::bench]
fn day03_part1_bench() {
    day03::process_part1(include_str!("../input.txt"));
}

#[divan::bench]
fn day03_part2_bench() {
    day03::process_part2(include_str!("../input.txt"));
}
