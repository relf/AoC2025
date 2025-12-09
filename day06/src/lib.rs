use ndarray::{Array1, Array2, Zip, s};

pub fn parse_input(input: &str) -> (Array2<usize>, Array1<char>) {
    let lines = input.lines().collect::<Vec<_>>();

    let n = lines.len();

    let numbers = lines[..n - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let numbers = Array2::from_shape_vec(
        (n - 1, numbers[0].len()),
        numbers.into_iter().flatten().collect(),
    )
    .unwrap();

    let ops = lines[n - 1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Array1<_>>();
    (numbers, ops)
}

pub fn process_part1(input: &str) -> usize {
    let (numbers, ops) = parse_input(input);
    Zip::from(&ops)
        .and(numbers.columns())
        .fold(0usize, |acc, &op, col| {
            let res = match op {
                '*' => col.product(),
                '+' => col.sum(),
                _ => unreachable!(),
            };
            acc + res
        })
}

pub fn process_part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();

    let n = lines.len() - 1;

    let ops = lines[n]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>();

    let numbers = lines[..n]
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let numbers = Array2::from_shape_vec(
        (n, numbers[0].len()),
        numbers.into_iter().flatten().collect(),
    )
    .unwrap();

    let mut window_sizes = vec![];
    let mut size = 1;
    (0..lines[0].len()).for_each(|i| {
        if numbers.column(i).iter().all(|c| c.is_whitespace()) {
            window_sizes.push(size);
            size = 1;
        } else {
            size += 1;
        }
    });
    window_sizes.push(size - 1);

    //dbg!(&window_sizes);

    window_sizes
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let size_acc = window_sizes.iter().take(i).sum::<usize>();
            let window = numbers.slice(s![.., size_acc..size_acc + s]);
            let op = ops[i];
            let col = window
                .columns()
                .into_iter()
                .map(|c| {
                    // dbg!(c);
                    if !c.iter().all(|ch| ch.is_whitespace()) {
                        c.iter()
                            .collect::<String>()
                            .trim()
                            .parse::<usize>()
                            .unwrap()
                    } else {
                        match op {
                            '*' => 1,
                            '+' => 0,
                            _ => unreachable!(),
                        }
                    }
                })
                .collect::<Array1<_>>();
            // dbg!(&col);
            match op {
                '*' => col.product(),
                '+' => col.sum(),
                _ => unreachable!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  "),
            4277556
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  "),
            3263827
        );
    }
}
