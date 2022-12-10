fn main() {
    let solver = day5::Parser {
        path: String::from("src/test.txt"),
    };
    println!("{}", solver.solve_pt1());
}
