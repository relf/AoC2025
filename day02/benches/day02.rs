fn main() {
    divan::main();
}

#[divan::bench]
fn day02_part1_bench() {
    day02::process_part1(include_str!("../input.txt"));
}

#[divan::bench]
fn day02_part2_bench() {
    day02::process_part2(include_str!("../input.txt"));
}
