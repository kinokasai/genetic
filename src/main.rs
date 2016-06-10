extern crate rand;

use rand::Rng;

/* Create a trait for method chaining */

trait Population {
    fn mate(&mut self) -> Self;
    fn print(&mut self) -> &mut Self;
    fn select(&mut self) -> Self;
    fn sort(&mut self) -> &mut Self;
}

impl Population for Vec<String> {

    fn print(&mut self) -> &mut Self {
        for individual in self.clone() {
            println!("{}", individual);
        }
        self
    }

    fn sort(&mut self) -> &mut Self {
        self.sort_by_key(|s| fitness(s));
        self
    }

    fn select(&mut self) -> Self {
        self.iter()
            .enumerate()
            .filter(|t| t.0 < (self.len() / 2))
            .map(|t| t.1)
            .cloned()
            .collect::<Self>()
    }

    fn mate(&mut self) -> Self {
        self.iter()
            .map(|idv| mate(idv, self))
            .fold(Vec::new(), |mut vec, t| {vec.push(t.0); vec.push(t.1); vec})
    }
}

/* Helper functions */

fn fitness(input : &String) -> usize {
    let model = "Life isn't about what you think it is.";
    input.chars().zip(model.chars())
        .fold(0, |sum, c| sum + (c.1 != c.0) as usize)
}

fn generate_string(len : usize) -> String {
    (0..len).map(|_| generate_char() as char)
        .collect()
}

fn generate_char() -> u8 {
    rand::thread_rng().gen_range(b' ', b'~')
}

fn mate(idv : &str, pop : &Vec<String>) -> (String, String) {
    let mate : &String =
        rand::thread_rng().choose(pop)
                           .unwrap();

    idv .chars()
        .zip(mate.chars())
        .map(|t| {
            if rand::thread_rng().gen::<bool>() {
                (t.1, t.0)
            } else {
                t
            }
        })
        .unzip()
}

fn populate(plen : usize, ilen : usize) -> Vec<String> {
    (0..plen)
        .map(|_| generate_string(ilen))
        .collect::<Vec<String>>()
}

fn main() {
    let mut pop = populate(1000, 38);

    let mut i = 0;

    loop {
        println!("{}", pop.sort()[0]);
        pop = pop.sort()
           .select()
           .sort()
           .mate();

        if pop.iter().fold(false, |b, idv| b | (fitness(idv) == 0)) { break; }

        i += 1;
    }
    println!("{}", pop.sort()[0]);
    println!("{} iterations.", i);
}
