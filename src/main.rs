use structopt::StructOpt;
use exitfailure::ExitFailure;
use rand::prelude::*;
use std::convert::TryInto;
use seir::build_human;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "n", long = "npeople", default_value = "3")]
    npeople: u32,
    #[structopt(short = "c", long = "cells", default_value = "4")]
    cells: u32,
}

fn get_occupant_by_id(pop: Vec<Human>, id: u32) {
    
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let initial_population = args.npeople;
    let cells = args.cells;
    println!("npeople={} cells={}", initial_population, cells);
    // if cells == 0 then use ncurses
    let mut grid = vec![0; (cells * cells).try_into().unwrap()];
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0, grid.len() - 1);
    // create vector of npeople humans and infect one
    let mut population = Vec::new();
    for n in 0..initial_population {
        let id = n + 1;
        let mut p = rng.sample(distr);
        for _ in 1..(grid.len()) {
            if grid[p] == 0 {
                break;
            }
            p = (p + 1) % grid.len();
            //println!("trying next cell for id={}", id);
        }
        let h = build_human(id);
        population.push(h);
        grid[p] = id;
    }
    population[0].infect();
    //println!("population={:?}", population);
    println!("grid={:?}", grid);
    for t in 0..3 {
        for me in &mut population {
            me.tick();
            println!("t={} h={}", t, me);
            /*
             * pick new location for human (NSEW)
             * if empty move there and zero old location
             * if occupied
             *  them.touched_by(me)
             * fn touched_by(other) - if 
             */
            let target_pos = 0; // some function old existing pos
            if grid[target_pos] == 0 {
                // move to target_pos
            } else {
                let them = population.get_occupant_by_id(grid[target_pos]);
                if them.infected() && me.susceptible() {
                    me.infect();
                    them.increment_r();
                } else if me.infected() && them.susceptible() {
                    them.infect();
                    me.increment_r();
                }
            }
        }
    }
    Ok(())
}
