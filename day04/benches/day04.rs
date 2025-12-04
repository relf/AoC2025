fn main() {
    divan::main();
}

#[divan::bench]
fn day04_part1_bench() {
    day04::process_part1(include_str!("../input.txt"));
}

#[divan::bench]
fn day04_part2_bench() {
    day04::process_part2(include_str!("../input.txt"));
}
