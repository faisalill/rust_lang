#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
    id: usize,
}

#[derive(Debug)]
struct Class {
    students: [Person; 3],
}

enum Character {
    Idle,
    Walking {
        x_direction: String,
        y_direction: String,
    },
    Running {
        speed: usize,
        x_direction: String,
        y_direction: String,
    },
    Attack {
        enemy: String,
        damange: usize,
    },
}

fn about_character(character: Character) {
    match character {
        Character::Idle => println!("Characterr is Idle"),
        Character::Attack { enemy, damange } => println!("{}'s hp is down by {} ", enemy, damange),
        Character::Walking {
            x_direction,
            y_direction,
        } => println!(
            "Walking in x: {} direction y: {} direction",
            x_direction, y_direction
        ),
        Character::Running {
            speed,
            x_direction,
            y_direction,
        } => println!(
            "Running in x: {} direction y: {} direction speed: {}",
            x_direction, y_direction, speed
        ),
    }
}

fn main() {
    let first_person: Person = Person {
        name: String::from("Jhon"),
        age: 23,
        id: 3334,
    };

    let second_person: Person = Person {
        name: String::from("Doe"),
        age: 34,
        id: 6666,
    };

    let third_person: Person = Person {
        name: String::from("Jhon Doe"),
        age: 88,
        id: 9999,
    };

    let first_class: Class = Class {
        students: [first_person, second_person, third_person],
    };

    println!("{:?}", first_class);
    println!("{:#?}", first_class);

    let idle_character = Character::Idle;
    let walking_character = Character::Walking {
        x_direction: String::from("Forward"),
        y_direction: String::from("Forward"),
    };
    let running_character = Character::Running {
        speed: 34,
        x_direction: String::from("Backward"),
        y_direction: String::from("East"),
    };
    let attack_character = Character::Attack {
        enemy: String::from("Common Soldier"),
        damange: 100,
    };

    about_character(idle_character);
    about_character(walking_character);
    about_character(running_character);
    about_character(attack_character);

    const const_val: usize = 34;

    static static_val: usize = 343;
}
