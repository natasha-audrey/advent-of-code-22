use color_eyre::eyre::Context;

/// # Test Input
/// ```
/// let solver = day4::Solver {
///   path: String::from("src/test.txt"),
/// };
/// assert_eq!(solver.solve_pt1(), 2);
/// assert_eq!(solver.solve_pt2(), 4);
/// ```
pub struct Solver {
    pub path: String,
}

#[derive(Clone, Copy)]
pub struct Assignment {
    min: i32,
    max: i32,
}

impl Assignment {
    pub fn new(assignment: &str) -> Assignment {
        let mut split = assignment.split("-").map(|x| x.parse::<i32>().unwrap());
        Assignment {
            min: split.next().unwrap(),
            max: split.next().unwrap(),
        }
    }

    pub fn contains(self, theirs: Assignment) -> bool {
        self.min <= theirs.min && self.max >= theirs.max
    }

    pub fn intersects(self, theirs: Assignment) -> bool {
        self.min <= theirs.max && theirs.min <= self.max
    }
}

impl Solver {
    pub fn read_input(&self) -> color_eyre::Result<String> {
        let input = std::fs::read_to_string(self.path.as_str())
            .wrap_err(format!("reading {}", self.path))?;

        Ok(input)
    }

    pub fn solve_pt1(&self) -> i32 {
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

    pub fn solve_pt2(&self) -> i32 {
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
