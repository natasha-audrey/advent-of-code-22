use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt").to_string();
    let solution = solve(input);
    let max_three = &solution[..3];
    println!("{:?}\nsum: {}", max_three, max_three.iter().sum::<u64>());

    Ok(())
}

fn solve(input: String) -> Vec<u64> {
    let groups = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .flatten()
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .collect_vec();

    groups
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input: String = String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
        );
        assert_eq!(
            solve(input),
            Vec::<u64>::from([24000, 11000, 10000, 6000, 4000])
        );
    }
}
