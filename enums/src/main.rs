fn main() {
    #[derive(Debug)]
    enum Direction {
        North,
        East,
        West,
        South,
    }

    impl Direction {
        fn name(&self) -> String {
            match self {
                Direction::North => String::from("North"),
                Direction::East => String::from("East"),
                Direction::South => String::from("South"),
                Direction::West => String::from("West"),
            }
        }

        fn turn_left(&self) -> Direction {
            println!("Turning left");
            match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            }
        }

        fn turn_right(&self) -> Direction {
            println!("Turning right");
            match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }
    }

    let mut direction = Direction::North;
    direction = direction.turn_left();
    println!("Current direction is: '{:?}'", direction);

    direction = direction.turn_right();
    println!("Current direction is: '{}'", direction.name());

    enum Animal {
        Dog(String),
        Cat { name: String, age: u8 },
        Bird,
    }

    impl Animal {
        fn name(&self) -> String {
            match self {
                Animal::Dog(name) => name.clone(),
                Animal::Cat { name, .. } => name.clone(),
                Animal::Bird => String::from("Nameless Birb"),
            }
        }

        fn age(&self) -> u8 {
            match self {
                Animal::Cat { age, .. } => *age,
                _ => 0,
            }
        }
    }

    let cat = Animal::Cat {
        name: String::from("Garfield"),
        age: 14,
    };
    println!("Cat '{}' is '{}' years old", cat.name(), cat.age());

    let dog = Animal::Dog(String::from("Mr. Barker"));
    println!("Dog '{}' is '{}' years old", dog.name(), dog.age());

    let birb = Animal::Bird;
    println!("Birb '{}' is '{}' years old", birb.name(), birb.age());
}
