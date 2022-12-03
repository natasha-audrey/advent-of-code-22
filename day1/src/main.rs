fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let solution = solve(input);
    let max_three = &solution[solution.len() - 3..];
    println!("{:?}\nsum: {}", max_three, max_three.iter().sum::<i32>());
}

fn solve(input: String) -> Vec<i32> {
    let split = input.split("\n");
    let mut sum: i32 = 0;
    let mut sums = Vec::<i32>::new();
    split.for_each(|i| {
        if i == "" {
            let pos = sums.binary_search(&sum).unwrap_or_else(|e| e);
            sums.insert(pos, sum);
            sum = 0;
        } else {
            sum += i.parse::<i32>().unwrap();
        }
    });
    return sums;
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
            Vec::<i32>::from([4000, 6000, 10000, 11000, 24000])
        );
    }
}
