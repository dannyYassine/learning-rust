use std::io;

struct BaseAnimal {
    name: String,
}
struct Sheep {
    base_props: BaseAnimal,
}
struct Cow {
    base_props: BaseAnimal,
}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
    fn get_name(&self) -> &String;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
    fn get_name(&self) -> &String {
        return &self.base_props.name;
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
    fn get_name(&self) -> &String {
        return &self.base_props.name;
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        return Box::new(Sheep {
            base_props: BaseAnimal {
                name: String::from("Sheep"),
            },
        });
    } else {
        return Box::new(Cow {
            base_props: BaseAnimal {
                name: String::from("Cow"),
            },
        });
    }
}

fn main() {
    println!("Please input your guess, or enter 'no' to quit:");

    loop {
        let mut random_number: String = String::new();

        io::stdin()
            .read_line(&mut random_number)
            .expect("Failed to read line");

        let answer = random_number.trim();

        if answer == "no" {
            println!("Quiting...");
            break;
        }

        let input: Result<f64, _> = answer.parse();

        if input.is_err() {
            println!("Unable to read... exiting.");
            break;
        }

        let animal = random_animal(input.unwrap());

        println!(
            "You've randomly chosen an animal, and it says {}",
            animal.noise()
        );
        println!("Animal name {}", animal.get_name());

        println!("Try again.");
    }
}
