fn main() {
    let solver = day5::Solver {
        path: String::from("src/input.txt"),
    };
    println!("{}", solver.solve_pt1());
}
