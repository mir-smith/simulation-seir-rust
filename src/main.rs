use structopt::StructOpt;
use exitfailure::ExitFailure;
use seir::build_human;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "population", default_value = "100")]
    population: u32,
    #[structopt(short = "c", long = "cells", default_value = "40")]
    cells: u32,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let population = args.population;
    let cells = args.cells;
    println!("population={} cells={}", population, cells);
    let mut h = build_human(7);
    h.infect();
    for t in 0..15 {
        h.tick();
        println!("t={} h={}", t, h);
    }
    Ok(())
}
