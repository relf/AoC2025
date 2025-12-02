fn is_invalid(id_str: i64) -> bool {
    let id_str = id_str.to_string();
    let mid = id_str.len() / 2;

    for w in 1..=mid {
        if id_str.len().is_multiple_of(w) {
            let mut invalid = true;
            for i in 0..(id_str.len() / w) - 1 {
                if id_str[(i + 1) * w..(i + 2) * w] != id_str[..w] {
                    invalid = false;
                    break;
                }
            }
            if invalid {
                return true;
            }
        }
    }
    false
}

pub fn run(input: &str) -> i64 {
    input.trim().split(',').fold(0, |acc, range| {
        let mut count = 0;
        let (start, end) = range.trim().split_once('-').unwrap();
        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();
        (start..=end).for_each(|id| {
            if is_invalid(id) {
                count += id
            }
        });
        acc + count
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day02b() {
        assert_eq!(run(include_str!("../input_test.txt")), 4174379265);
    }
}
