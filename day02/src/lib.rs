fn is_invalid_part1(id_str: i64) -> bool {
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

fn is_invalid_part2(id_str: i64) -> bool {
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

fn process(ic: fn(i64) -> bool, input: &str) -> i64 {
    input.trim().split(',').fold(0, |acc, range| {
        let mut count = 0;
        let (start, end) = range.trim().split_once('-').unwrap();
        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();
        (start..=end).for_each(|id| {
            if ic(id) {
                count += id
            }
        });
        acc + count
    })
}

pub fn process_part1(input: &str) -> i64 {
    process(is_invalid_part1, input)
}

pub fn process_part2(input: &str) -> i64 {
    process(is_invalid_part2, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,\
        2121212118-2121212124"
            ),
            1227775554
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,\
        2121212118-2121212124"
            ),
            4174379265
        );
    }
}
