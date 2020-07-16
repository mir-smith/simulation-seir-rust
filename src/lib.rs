use std::fmt;
use std::convert::TryInto;
use rand::Rng;
use rand::rngs::ThreadRng;

/* Health */

#[derive(Debug, Copy, Clone)]
pub enum Health {
    Susceptible,
    Exposed,
    Infected,
    Recovered,
}

/* Human */

pub struct Human {
    id: usize,
    health: Health,
    position: usize,
    tick_trigger: isize,
    infected_by :usize,
}

pub fn create_human(id: usize, position: usize) -> Human {
    Human {
        id,
        health: Health::Susceptible,
        position,
        tick_trigger: 0,
        infected_by: 0,
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Human id={} health={} position={} tick_trigger={} infected_by={}>",
            self.id, self.status(), self.position, self.tick_trigger, self.infected_by)
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

    fn infect(&mut self) {
        match self.health {
            Health::Susceptible => {
                self.health = Health::Exposed;
                self.tick_trigger = -3;
            }
            _ => (),
        }
    }
    
    fn infected(&self) -> bool {
        match self.health {
            Health::Infected => true,
            _ => false,
        }
    }

    fn susceptible(&self) -> bool {
        match self.health {
            Health::Susceptible => true,
            _ => false,
        }
    }    
}

/* Touched */

#[derive(Debug, Copy, Clone)]
pub struct Touch {
    toucher: usize,
    health: Health,
    touched: usize,
}

/* Model */

pub struct Model {
    population: Vec<Human>,
    grid: Vec<usize>,
    rng: ThreadRng,
    grid_width: usize,
    grid_size: usize,
}

pub fn create_model(npeople: usize, cells: usize) -> Model {
    let grid_size = cells * cells;
    let mut population = Vec::new();
    let mut grid = vec![0; (grid_size).try_into().unwrap()];
    let mut rng = rand::thread_rng();
    for n in 0..npeople {
        let id = n + 1;
        let mut p = rng.gen_range(0, grid.len());
        for _ in 1..(grid.len()) {
            if grid[p] == 0 {
                break;
            }
            p = (p + 1) % grid.len();
            println!("trying next cell for id={}", id);
        }
        let h = create_human(id, p);
        population.push(h);
        grid[p] = id;
        println!("placing id={} at position={}", id, p);
    }
    population[0].infect();
    Model {
        population,
        grid,
        rng,
        grid_width: cells,
        grid_size,
    }
}

impl Model {
    pub fn get_human_by_id(&mut self, id: usize) -> &mut Human {
        &mut self.population[id - 1]
    }

    pub fn tick(&mut self, _t: usize) {
        let mut touches: Vec<Touch> = Vec::new();
        for me in &mut self.population {
            me.tick();
            let mut target_pos = me.position;
            let direction = self.rng.gen_range(0, 4);
            match direction {
                0 => target_pos = target_pos + 1,
                1 => target_pos = target_pos + self.grid_size - 1,
                2 => target_pos = target_pos + self.grid_width,
                3 => target_pos = target_pos + self.grid_size - self.grid_width,
                _ => panic!("direction random number outside range"),
            }
            target_pos = target_pos % self.grid_size;
            if self.grid[target_pos] == 0 {
                self.grid[me.position] = 0;
                me.position = target_pos;
                self.grid[me.position] = me.id;
            } else {
                let their_id = self.grid[target_pos];
                if me.infected() {
                    touches.push(Touch { toucher: me.id, health: me.health, touched: their_id });
                }
            }

        }
        for touch in touches {
            let touched = self.get_human_by_id(touch.touched);
            if touched.susceptible() {
                touched.infect();
                println!("***** {} has infected {}", touch.toucher, touch.touched)
            }
        }
    }

    pub fn display(&self) {
        let mut counts: [usize; 4] = [0; 4];
        for me in &self.population {
            match me.health {
                Health::Susceptible => counts[0] += 1,
                Health::Exposed     => counts[1] += 1,
                Health::Infected    => counts[2] += 1,
                Health::Recovered   => counts[3] += 1,
            }
            println!("{:#}", me);
        }
        println!("{:?}", counts);
        for id in &self.grid {
            print!("{:#} ", id);
        }
        println!("\n---------");
        if counts[1] + counts[2] < 1 {
            panic!("no more transmission")
        }
    }
}

/* Tests */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
