
pub fn run(input: &str) -> i32 {
    let mut passwd = 0;
    let _ = input
        .lines()
        .map(|s| {
            let (r, d) = s.split_at(1);
            let dir = if r == "L" { -1 } else { 1 };
            dir * d.parse::<i32>().unwrap()
        })
        .fold(50, |acc, x| {
            let acc = (acc + x) % 100;
            if acc == 0 {
                passwd += 1;
            }
            acc
        });

    passwd
}

pub fn main() {
    let passwd = run(include_str!("../input.txt"));
    assert_eq!(passwd, 1182);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01a() {
        assert_eq!(run(include_str!("../input_test.txt")), 3);
    }
}