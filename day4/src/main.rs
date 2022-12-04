use color_eyre::eyre::Context;

fn main() {
    let solver = Solver {
        path: String::from("src/input.txt"),
    };
    let solution = solver.solve_pt1();
    println!("{solution}");
    let solution = solver.solve_pt2();
    println!("{solution}");
}

struct Solver {
    path: String,
}

#[derive(Clone, Copy)]
struct Assignment {
    min: i32,
    max: i32,
}

impl Assignment {
    fn new(assignment: &str) -> Assignment {
        let mut split = assignment.split("-").map(|x| x.parse::<i32>().unwrap());
        Assignment {
            min: split.next().unwrap(),
            max: split.next().unwrap(),
        }
    }

    fn contains(self, theirs: Assignment) -> bool {
        self.min <= theirs.min && self.max >= theirs.max
    }

    fn intersects(self, theirs: Assignment) -> bool {
        self.min <= theirs.max && theirs.min <= self.max
    }
}

impl Solver {
    fn read_input(&self) -> color_eyre::Result<String> {
        let input = std::fs::read_to_string(self.path.as_str())
            .wrap_err(format!("reading {}", self.path))?;

        Ok(input)
    }

    fn solve_pt1(&self) -> i32 {
        let input = self.read_input().unwrap();
        let mut contains = 0;
        input.lines().for_each(|line| {
            let mut split = line.split(",");
            let (first, second) = (
                Assignment::new(split.next().unwrap()),
                Assignment::new(split.next().unwrap()),
            );
            if first.contains(second) {
                contains += 1;
            } else if second.contains(first) {
                contains += 1;
            }
        });
        contains
    }

    fn solve_pt2(&self) -> i32 {
        let input = self.read_input().unwrap();
        let mut intersects = 0;
        input.lines().for_each(|line| {
            let mut split = line.split(",");
            let (first, second) = (
                Assignment::new(split.next().unwrap()),
                Assignment::new(split.next().unwrap()),
            );
            if first.intersects(second) {
                intersects += 1;
            }
        });
        intersects
    }
}

mod tests {
    #[test]
    fn test_input() {
        let solver = crate::Solver {
            path: String::from("src/test.txt"),
        };
        assert_eq!(solver.solve_pt1(), 2);
        assert_eq!(solver.solve_pt2(), 4);
    }
}
