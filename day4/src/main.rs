fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let solver = day4::Solver {
        path: String::from("src/input.txt"),
    };
    let solution = solver.solve_pt1();
    println!("{solution}");
    let solution = solver.solve_pt2();
    println!("{solution}");

    Ok(())
}
