fn read_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

pub fn process_part1(input: &str) -> usize {
    let (ranges_str, numbers_str) = input.split_once("\n\n").unwrap();
    let ranges = read_ranges(ranges_str);
    numbers_str
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .filter(|&num| {
            ranges
                .iter()
                .any(|&(start, end)| num >= start && num <= end)
        })
        .count()
}

pub fn process_part2(input: &str) -> usize {
    let (ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges = read_ranges(ranges_str);

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut acc = vec![ranges[0]];
    for range in ranges[1..].iter() {
        if range.0 <= acc.last().unwrap().1 {
            let last = acc.pop().unwrap();
            acc.push((last.0.min(range.0), last.1.max(range.1)));
        } else {
            acc.push(*range);
        }
    }
    acc.iter().map(|(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32"),
            3
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32"),
            14
        );
    }
}
