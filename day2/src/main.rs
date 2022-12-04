use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cfg = Config {
        path: String::from("src/input.txt"),
    };
    let pt1 = cfg.solve_pt1().unwrap();
    let pt2 = cfg.solve_pt2().unwrap();
    
    println!("{pt1}\n{pt2}");

    Ok(())
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

    fn solve_pt1(&self) -> color_eyre::Result<i32> {
        let input = self.read_input().unwrap();

        let mut sum = 0;

        input.lines().for_each(|line| {
            let mut round = line.split(' ');
            let opp_choice = match round.next().unwrap() {
                "A" => Ok(Move::Rock),
                "B" => Ok(Move::Paper),
                "C" => Ok(Move::Scissors),
                _ => Err("invalid input"),
            }
            .unwrap();
            let my_choice = match round.next().unwrap() {
                "X" => Ok(Move::Rock),
                "Y" => Ok(Move::Paper),
                "Z" => Ok(Move::Scissors),
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

    fn solve_pt2(&self) -> color_eyre::Result<i32> {
        let input = self.read_input().unwrap();

        let mut sum = 0;

        input.lines().for_each(|line| {
            let mut round = line.split(' ');
            let theirs = match round.next().unwrap() {
                "A" => Ok(Move::Rock),
                "B" => Ok(Move::Paper),
                "C" => Ok(Move::Scissors),
                _ => Err("invalid input"),
            }
            .unwrap();
            let ours = match round.next().unwrap() {
                "X" => Ok(RoundResult::Loss),
                "Y" => Ok(RoundResult::Draw),
                "Z" => Ok(RoundResult::Win),
                _ => Err("invalid input"),
            }
            .unwrap();

            let our_move = ours.matching_move(theirs);
            sum += Round {
                my_choice: our_move,
                opp_choice: theirs,
            }.play()
        });

        Ok(sum)
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl RoundResult {
    fn matching_move(self, theirs: Move) -> Move {
        match self {
            RoundResult::Loss => theirs.losing_move(),
            RoundResult::Draw => theirs,
            RoundResult::Win => theirs.winning_move(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

impl Move {
    const ALL_MOVES: [Move;3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES.iter().copied().find(|m| m.wins(self)).expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES.iter().copied().find(|&m| self.wins(m)).expect("at least one move beats us")
    }

    fn wins(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> RoundResult {
        if self.wins(theirs) {
            RoundResult::Win
        } else if theirs.wins(self) {
            RoundResult::Loss
        } else {
            RoundResult::Draw
        }
    }
}

#[derive(Debug)]

struct Round {
    my_choice: Move,
    opp_choice: Move,
}

impl Round {
    fn play(&self) -> i32 {
        let result = self.my_choice.outcome(self.opp_choice);

        result as i32 + self.my_choice as i32
    }
}

mod tests {
    #[test]
    fn test_input() {
        let cfg = crate::Config {
            path: String::from("src/test.txt"),
        };
        assert_eq!(cfg.solve_pt1().ok(), Some(15));
        assert_eq!(cfg.solve_pt2().ok(), Some(12));
    }
    #[test]
    fn given_input() {
        let cfg = crate::Config {
            path: String::from("src/input.txt"),
        };
        assert_eq!(cfg.solve_pt1().ok(), Some(13484));
    }
}
