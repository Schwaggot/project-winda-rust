use crate::person;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::seq::SliceRandom;
use person::Gender;


#[derive(Debug)]
pub(crate) struct NameGenerator {
    male_names: Vec<String>,
    female_names: Vec<String>,
}

impl NameGenerator {
    pub fn new() -> NameGenerator {
        NameGenerator {
            male_names: Self::load_names(Gender::Male),
            female_names: Self::load_names(Gender::Female),
        }
    }

    fn load_names(gender: Gender) -> Vec<String> {
        let file_name = if gender == Gender::Male { "male_names.txt" } else { "female_names.txt" };
        let file_path = format!("data/cmu-name-corpus/{}", file_name);

        let mut names = Vec::new();
        // File must exist in current path before this produces output
        // see https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
        if let Ok(lines) = Self::read_lines(file_path) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    if ip.len() > 0 && !ip.starts_with("#") {
                        names.push(ip);
                    }
                }
            }
        }
        names
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    // see https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn generate_name(self, gender: Gender) -> String {
        if gender == Gender::Male {
            self.male_names.choose(&mut rand::thread_rng()).unwrap().clone()
        } else {
            self.female_names.choose(&mut rand::thread_rng()).unwrap().clone()
        }
    }
}

