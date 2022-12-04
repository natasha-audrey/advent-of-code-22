use color_eyre::eyre::Context;

fn main() {
    let solution = Config {
        path: String::from("src/input.txt"),
    }
    .solve()
    .unwrap();
    println!("{solution}")
}

struct Config {
    path: String,
}

impl Config {
    fn read_input(&self) -> color_eyre::Result<String> {
        let input = std::fs::read_to_string(self.path.as_str())
            .wrap_err(format!("reading {}", self.path))?;

        Ok(input)
    }

    pub fn solve(&self) -> color_eyre::Result<i32> {
        color_eyre::install()?;
        let input = self.read_input().unwrap();

        let mut sum = 0;

        input.lines().for_each(|line| {
            let mut round = line.split(' ');
            let opp_choice = match round.next().unwrap() {
                "A" => Ok(Shape::Rock),
                "B" => Ok(Shape::Paper),
                "C" => Ok(Shape::Scissors),
                _ => Err("invalid input"),
            }
            .unwrap();
            let my_result = match round.next().unwrap() {
                "X" => Ok(RoundResult::Loss),
                "Y" => Ok(RoundResult::Draw),
                "Z" => Ok(RoundResult::Win),
                _ => Err("invalid input"),
            }
            .unwrap();

            let res = Round {
                my_result,
                opp_choice,
            };
            sum += res.play();
        });

        Ok(sum)
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    Loss = 0,
    Draw = 3,
    Win  = 6,
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper,
    Scissors,
}

#[derive(Debug)]

struct Round {
    my_result: RoundResult,
    opp_choice: Shape,
}

impl Round {
    fn play(&self) -> i32 {
        let result = match (self.my_result, self.opp_choice) {
            (RoundResult::Loss, Shape::Rock) => Shape::Scissors,
            (RoundResult::Loss, Shape::Paper) => Shape::Rock,
            (RoundResult::Loss, Shape::Scissors) => Shape::Paper,
            (RoundResult::Draw, _) => self.opp_choice,
            (RoundResult::Win, Shape::Rock) => Shape::Paper,
            (RoundResult::Win, Shape::Paper) => Shape::Scissors,
            (RoundResult::Win, Shape::Scissors) => Shape::Rock,
        };

        result as i32 + self.my_result as i32
    }

}

mod tests {
    #[test]
    fn is_solved() {
        let cfg = crate::Config {
            path: String::from("src/test.txt"),
        };
        assert_eq!(cfg.solve().ok(), Some(12))
    }
}
