fn find_max_batt_index(bank: &[u8]) -> usize {
    let mut index = 0;
    'outer: for c in [b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2', b'1'] {
        for (i, b) in bank.iter().enumerate() {
            if *b == c {
                index = i;
                break 'outer;
            }
        }
    }
    index
}

fn compute_bank_joltage(bank: &[u8], n: usize) -> i64 {
    let n = n - 1;
    let bound = bank.len() - n;
    let idx = find_max_batt_index(&bank[..bound]);

    let jolt_n = (bank[idx] as char).to_digit(10).unwrap();

    if n > 0 {
        10_i64.pow(n as u32) * jolt_n as i64 + compute_bank_joltage(&bank[idx + 1..], n)
    } else {
        jolt_n as i64
    }
}

pub fn process_part1(input: &str) -> i64 {
    input
        .lines()
        .map(|b| compute_bank_joltage(b.as_bytes(), 2))
        .sum()
}

pub fn process_part2(input: &str) -> i64 {
    input
        .lines()
        .map(|b| compute_bank_joltage(b.as_bytes(), 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1("987654321111111\n811111111111119\n234234234234278\n818181911112111"),
            357
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2("987654321111111\n811111111111119\n234234234234278\n818181911112111"),
            3121910778619
        );
    }
}
