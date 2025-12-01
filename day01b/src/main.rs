pub fn run(input: &str) -> i32 {
    let mut passwd = 0;
    let _ = input
        .lines()
        .map(|s| {
            let (r, d) = s.split_at(1);
            let dir = if r == "L" { -1 } else { 1 };
            dir * d.parse::<i32>().unwrap()
        })
        .fold(50, |acc, dial| {
            // Each full 100 steps adds one to the password
            let q = (dial.abs() - 1) / 100;
            passwd += q;

            // Check if we cross the 0 boundary
            let p = acc + dial % 100;
            if p.abs() > 100 || acc * p < 0 {
                passwd += 1;
            }

            // Update the accumulator modulo 100
            let acc = (acc + dial) % 100;
            // Handle the case where we land exactly on 0
            if acc == 0 {
                passwd += 1;
            }
            acc
        });

    passwd
}

pub fn main() {
    let passwd = run(include_str!("../input.txt"));
    assert_eq!(passwd, 6907);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01b() {
        assert_eq!(run(include_str!("../input_test.txt")), 6);
    }
}
