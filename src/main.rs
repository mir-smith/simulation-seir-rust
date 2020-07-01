use std::fmt;
use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "population", default_value = "100")]
    population: u32,
    #[structopt(short = "c", long = "cells", default_value = "40")]
    cells: u32,
}

#[derive(Debug)]
enum Health {
    Susceptible,
    Exposed,
    Infected,
    Recovered,
}

struct Human {
    id: u32,
    health: Health,
    tick_trigger: i32,
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Human id={} health={} tick_trigger={}", self.id, self.status(), self.tick_trigger)
    }
}

fn build_human(id: u32) -> Human {
    Human {
        id,
        health: Health::Susceptible,
        tick_trigger: 0,
    }
}

impl Human {
    fn status(&self) -> &str {
        match self.health {
            Health::Susceptible => "Susceptible",
            Health::Exposed => "Exposed",
            Health::Infected => "Infected",
            Health::Recovered => "Recovered",
        }
    }

    fn infect(&mut self) {
        match self.health {
            Health::Susceptible => {
                self.health = Health::Exposed;
                self.tick_trigger = -3;
            }
            _ => (),
        }
    }

    fn tick(&mut self) {
        self.tick_trigger += 1;
        if self.tick_trigger == 0 {
            match self.health {
                Health::Exposed => {
                    self.health = Health::Infected;
                    self.tick_trigger = -6;
                }
                Health::Infected => self.health = Health::Recovered,
                _ => (),
            }
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let population = args.population;
    let cells = args.cells;
    println!("population={} cells={}", population, cells);
    let mut h = build_human(7);
    //println!("Human id={} health={}", h.id, h.status());
    h.infect();
    //println!("Human id={} health={}", h.id, h.status());
    for t in 0..15 {
        h.tick();
        println!("t={} h={}", t, h);
        //println!("t={} Human id={} health={} tick_trigger={}", t, h.id, h.status(), h.tick_trigger);
    }
    Ok(())
}
