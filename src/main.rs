use structopt::StructOpt;
use exitfailure::ExitFailure;
use population::create_model;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "n", long = "npeople", default_value = "200")]
    npeople: usize,
    #[structopt(short = "c", long = "cells", default_value = "20")]
    cells: usize,
    #[structopt(short = "t", long = "ticks", default_value = "100")]
    ticks: usize,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut model = create_model(args.npeople, args.cells);
    for t in 0..args.ticks {
        model.tick(t);
        model.display();
    }
    Ok(())
}
