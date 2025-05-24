pub enum Animal {
    // Define the Animal variants here
    Dog,
    Cat(String),
    Bird { species: String, can_fly: bool },
}

pub fn describe_animal(animal: &Animal) -> String {
    match animal{
        Animal::Dog => "A friendly dog.".to_string(),
        Animal::Cat(name) => format!("A cat named {name}."),
        Animal::Bird{species, can_fly} => format!("A {species} that {} fly.", if *can_fly {"can"} else {"cannot"})
    }
}

// Example use case
pub fn main() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}
