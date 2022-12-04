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
            let my_choice = match round.next().unwrap() {
                "X" => Ok(Shape::Rock),
                "Y" => Ok(Shape::Paper),
                "Z" => Ok(Shape::Scissors),
                _ => Err("invalid input"),
            }
            .unwrap();

            let res = Round {
                my_choice,
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
    my_choice: Shape,
    opp_choice: Shape,
}

impl Round {
    fn play(&self) -> i32 {
        let result = match (self.my_choice, self.opp_choice) {
            (Shape::Rock, Shape::Rock) => RoundResult::Draw,
            (Shape::Rock, Shape::Paper) => RoundResult::Loss,
            (Shape::Rock, Shape::Scissors) => RoundResult::Win,
            (Shape::Paper, Shape::Rock) => RoundResult::Win,
            (Shape::Paper, Shape::Paper) => RoundResult::Draw,
            (Shape::Paper, Shape::Scissors) => RoundResult::Loss,
            (Shape::Scissors, Shape::Rock) => RoundResult::Loss,
            (Shape::Scissors, Shape::Paper) => RoundResult::Win,
            (Shape::Scissors, Shape::Scissors) => RoundResult::Draw,
        };

        result as i32 + self.my_choice as i32
    }
}

mod tests {
    #[test]
    fn is_solved() {
        let cfg = crate::Config {
            path: String::from("src/test.txt"),
        };
        assert_eq!(cfg.solve().ok(), Some(15))
    }
}
