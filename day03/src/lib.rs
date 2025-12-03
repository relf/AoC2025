fn find_max_index(bank: &str) -> usize {
    let mut index = 0;
    'outer: for c in ['9', '8', '7', '6', '5', '4', '3', '2', '1'] {
        for (i, b) in bank.chars().enumerate() {
            if b == c {
                index = i;
                break 'outer;
            }
        }
    }
    index
}

fn process_bank(bank: &str) -> i64 {
    let t_idx = find_max_index(&bank[..bank.len() - 1]);
    let u_idx = t_idx + 1 + find_max_index(&bank[t_idx + 1..]);

    let bank = bank.as_bytes();
    let t = (bank[t_idx] as char).to_digit(10).unwrap();
    let u = (bank[u_idx] as char).to_digit(10).unwrap();
    (10 * t + u) as i64
}

pub fn process_part1(input: &str) -> i64 {
    input.lines().map(process_bank).sum()
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
}
