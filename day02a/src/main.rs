fn is_invalid(id_str: i64) -> bool {
    let id_str = id_str.to_string();
    if !id_str.len().is_multiple_of(2) {
        return false;
    } else {
        let mid = id_str.len() / 2;
        if id_str[..mid] == id_str[mid..] {
            return true;
        }
    }
    false
}

fn run(input: &str) -> i64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut count = 0;
            let (start, end) = range.trim().split_once('-').unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            (start..=end).for_each(|id| {
                if is_invalid(id) {
                    count += id
                }
            });
            count
        })
        .sum()
}

fn main() {
    let result = run(include_str!("../input.txt"));
    assert_eq!(result, 34826702005);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day02a() {
        assert_eq!(run(include_str!("../input_test.txt")), 1227775554);
    }
}
