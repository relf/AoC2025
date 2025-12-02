fn is_invalid(id_str: i64) -> bool {
    let id_str = id_str.to_string();
    let mid = id_str.len() / 2;

    for w in 1..=mid {
        if id_str.len().is_multiple_of(w) {
            let r = format!(r"({}){{{}}}", &id_str[..w].to_string(), id_str.len() / w);
            let re = regex::Regex::new(&r).unwrap();
            if re.is_match(&id_str) {
                return true;
            }
        }
    }
    false
}

pub fn run(input: &str) -> i64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut count = 0;
            let (start, end) = range.trim().split_once('-').unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            println!("Processing range: {}-{}", start, end);
            (start..=end).for_each(|id| {
                //dbg!(id);
                if is_invalid(id) {
                    //println!("Found invalid id: {}", id);
                    count += id
                }
            });
            count
        })
        .sum()
}

pub fn main() {
    let result = run(include_str!("../input.txt"));
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day02b() {
        assert_eq!(run(include_str!("../input_test.txt")), 4174379265);
    }
}
