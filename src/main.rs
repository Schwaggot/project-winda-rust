mod naming;
mod elevator;
mod person;
mod entity;

use rand::Rng;
use crate::naming::NameGenerator;
use crate::elevator::Elevator;
use crate::entity::Entity;
use crate::person::Person;
use crate::person::PersonBuilder;
use crate::person::Gender;
use std::time::{Duration, SystemTime};


#[derive(Copy, Clone, Debug, PartialEq)]
enum LocationType {
    Building,
    Floor,
    Elevator,
}


trait Location: Sized {
    fn location_type() -> LocationType;
}


fn main() {
    let mut num_males = 0;
    let mut num_females = 0;

    for i in 0..100 {
        let person = PersonBuilder::new().finish();
        println!("{:?}", person);

        if person.gender == Gender::Male {
            num_males += 1;
        } else {
            num_females += 1;
        }
    }

    println!("Generated {:?} people ({:?} female and {:?} male)", num_males + num_females, num_females, num_males);

    let mut elevator: Elevator = Elevator::new();
    elevator.direction = 1.0;

    println!("Start: {}", elevator);

    let start = SystemTime::now();
    let mut last_time = start.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let mut tick = 0;

    println!("time\taltitude\tvelocity");
    loop {
        let current = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let elapsed = current - last_time;

        elevator.simulate(elapsed);

        // if tick % 10000000 == 0 {
        //     println!("{} ({:.3}s, elapsed={:?})", elevator, start.elapsed().unwrap().as_secs_f32(), elapsed);
        // }
        if tick % 10000000 == 0 {
            println!("{:.3}\t{:.3}\t{:.3}", start.elapsed().unwrap().as_secs_f32(), elevator.altitude, elevator.velocity);
        }

        tick = tick + 1;
        last_time = current;
    }
}
