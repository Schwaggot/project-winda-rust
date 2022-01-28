mod naming;

use rand::Rng;


#[derive(Copy, Clone, Debug, PartialEq)]
enum DoorState {
    Closed,
    Closing,
    Open,
    Opening,
}

struct Elevator {
    max_weight: f32,
    current_weight: f32,
    velocity: f32,
    door_state: DoorState,
    height: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Gender {
    Male,
    Female,
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum LocationType {
    Building,
    Floor,
    Elevator,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum EntityType {
    Elevator,
    Person,
}

trait Location: Sized {
    fn location_type() -> LocationType;
}

trait Entity: Sized {
    fn entity_type() -> EntityType;
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: Gender,
    age: u8,
    weight: f32,
    height: u8,
}

#[derive(Debug)]
struct PersonBuilder {
    name: Option<String>,
    gender: Option<Gender>,
    age: Option<u8>,
    weight: Option<f32>,
    height: Option<u8>,
}

impl PersonBuilder {
    fn new() -> PersonBuilder {
        PersonBuilder {
            name: None,
            gender: None,
            age: None,
            weight: None,
            height: None,
        }
    }

    fn name(mut self, name: String) -> PersonBuilder {
        self.name = Some(name);
        self
    }

    fn gender(mut self, gender: Gender) -> PersonBuilder {
        self.gender = Some(gender);
        self
    }

    fn age(mut self, age: u8) -> PersonBuilder {
        self.age = Some(age);
        self
    }

    fn weight(mut self, weight: f32) -> PersonBuilder {
        self.weight = Some(weight);
        self
    }

    fn height(mut self, height: u8) -> PersonBuilder {
        self.height = Some(height);
        self
    }

    fn finish(mut self) -> Person {
        let mut rng = rand::thread_rng();

        // if no name supplied generate random name
        let name = match self.name.clone() {
            Some(x) => x,
            None => "Meow".to_string()
        };

        // if no gender is supplied randomly select one
        let gender = match self.gender {
            Some(x) => x,
            None => if rng.gen_range(0, 1) == 0 { Gender::Male } else { Gender::Female }
        };

        // if no age is supplied calculate random age between 6 (capable of walking and pressing
        // elevator buttons) and 99 (still able to walk on its own)
        let age = match self.age {
            Some(x) => x,
            None => rng.gen_range(6, 99)
        };

        // if no weight supplied calculate height based on age and gender
        // TODO educated guess or formula
        let height = match self.height {
            Some(x) => x,
            None => rng.gen_range(150, 190)
        };

        // if no weight supplied calculate weight based on Hamwi method,
        // see https://en.wikipedia.org/wiki/Human_body_weight#Hamwi_method
        // TODO add variation
        let weight = match self.weight {
            Some(x) => x,
            None => if gender == Gender::Male {
                48.0 + 1.1 * (height as f32 - 152.0)
            } else {
                45.4 + 0.9 * (height as f32 - 152.0)
            }
        };

        Person {
            name,
            gender,
            age,
            weight,
            height,
        }
    }
}

fn main() {
    let p1 = PersonBuilder::new().finish();
    let p2 = PersonBuilder::new().finish();
    println!("{:?}", p1);
    println!("{:?}", p2);

    let name_generator = naming::NameGenerator::new();
}
