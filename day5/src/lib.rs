use std::fmt;

use color_eyre::eyre::Context;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

/// # Test Input
/// ```
/// let parser = day5::Parser {
///   path: String::from("src/test.txt"),
/// };
/// assert_eq!(parser.solve_pt1(), "CMZ");
/// ```
/// ```
/// let parser = day5::Parser {
///   path: String::from("src/test.txt"),
/// };
/// assert_eq!(parser.solve_pt2(), "MCD");
/// ```
/// ```
/// let parser = day5::Parser {
///   path: String::from("src/input.txt"),
/// };
/// assert_eq!(parser.solve_pt1(), "HBTMTBSDC");
/// ```
/// ```
/// let parser = day5::Parser {
///   path: String::from("src/input.txt"),
/// };
/// assert_eq!(parser.solve_pt2(), "PQTJRSHWS");
/// ```
pub struct Parser {
    pub path: String,
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl Default for Crate {
    fn default() -> Self {
        Self('_')
    }
}

struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        writeln!(
            f,
            "{}",
            self.0
                .iter()
                .map(|pile| pile.last().copied().unwrap_or_default())
                .join("")
        )?;
        Ok(())
    }
}

impl Piles {
    fn from(v: Vec<Vec<Option<Crate>>>) -> Self {
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        Self(
            (0..len)
                .map(|_| {
                    iters
                        .iter_mut()
                        .rev()
                        .filter_map(|n| n.next().unwrap())
                        .collect::<Vec<Crate>>()
                })
                .collect(),
        )
    }

    fn apply(&mut self, ins: Instruction) {
        println!("applying {ins:?}\nbefore: {self:?}");
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
        println!("after: {self:?}");
    }

    fn apply_v2(&mut self, ins: Instruction) {
        println!("applying {ins:?}\nbefore: {self:?}");
        let mut a: Vec<Crate> = Vec::new();
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            a.push(el);
        }
        a.reverse();
        self.0[ins.dst].extend(a.iter());
        println!("after: {self:?}");
    }
}

impl Parser {
    pub fn read_input(&self) -> color_eyre::Result<String> {
        let input = std::fs::read_to_string(self.path.as_str())
            .wrap_err(format!("reading {}", self.path))?;

        Ok(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        fn parse_number(i: &str) -> IResult<&str, usize> {
            map_res(digit1, |s: &str| s.parse::<usize>())(i)
        }
        map(
            tuple((
                preceded(tag("move "), parse_number),
                preceded(tag(" from "), parse_number),
                preceded(tag(" to "), parse_number),
            )),
            |(quantity, src, dst)| Instruction {
                quantity,
                src: src - 1,
                dst: dst - 1,
            },
        )(input)
    }

    fn parse_crate(input: &str) -> IResult<&str, Crate> {
        let first_char = |s: &str| Crate(s.chars().next().unwrap());
        let f = delimited(tag("["), take(1_usize), tag("]"));
        map(f, first_char)(input)
    }

    fn parse_crate_option(input: &str) -> IResult<&str, Option<Crate>> {
        alt((map(Self::parse_crate, Some), map(tag("   "), |_| None)))(input)
    }

    fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
        separated_list1(tag(" "), Self::parse_crate_option)(i)
    }

    pub fn solve_pt1(self) -> String {
        let input = self.read_input().unwrap();
        let mut lines = input.lines();

        let crate_lines: Vec<Vec<Option<Crate>>> = (&mut lines)
            .map_while(|line| {
                all_consuming(Self::parse_crate_line)(line)
                    .finish()
                    .ok()
                    .map(|(_, line)| line)
            })
            .collect();
        let mut piles = Piles::from(crate_lines);

        let instructions = lines.filter(|x| !x.is_empty()).map(|inst| {
            let (_, i) = Self::parse_instruction(inst).unwrap();
            i
        });

        for ins in instructions {
            piles.apply(ins);
        }

        piles
            .0
            .iter()
            .map(|pile| pile.last().copied().unwrap_or_default())
            .join("")
    }

    pub fn solve_pt2(self) -> String {
        let input = self.read_input().unwrap();
        let mut lines = input.lines();

        let crate_lines: Vec<Vec<Option<Crate>>> = lines
            .by_ref()
            .map_while(|line| {
                all_consuming(Self::parse_crate_line)(line)
                    .finish()
                    .ok()
                    .map(|(_, line)| line)
            })
            .collect();
        let mut piles = Piles::from(crate_lines);

        let instructions = lines.filter(|x| !x.is_empty()).map(|inst| {
            let (_, i) = Self::parse_instruction(inst).unwrap();
            i
        });

        for ins in instructions {
            piles.apply_v2(ins);
        }

        piles
            .0
            .iter()
            .map(|pile| pile.last().copied().unwrap_or_default())
            .join("")
    }
}
