fn main() {
    divan::main();
}

#[divan::bench]
fn day06_part1_bench() {
    day06::process_part1(include_str!("../input.txt"));
}

#[divan::bench]
fn day06_part2_bench() {
    day06::process_part2(include_str!("../input.txt"));
}
