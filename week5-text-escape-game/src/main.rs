use std::io;

enum Room {
    Start,
    Room1,
    Room2,
    Room3,
    End,
}

fn main() {
    let mut current_room = Room::Start;
    // a boolean to keep track of whether the player has the key
    let mut has_key = false;
    println!("You are in the starting room. Choose your path.");
    loop {
        match current_room {
            Room::Start => {
                println!("You see three doors in front of you.");
                println!("1. Door to Room 1");
                println!("2. Door to Room 2");
                println!("3. Door to Room 3");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input.trim() {
                    "1" => current_room = Room::Room1,
                    "2" => current_room = Room::Room2,
                    "3" => current_room = Room::Room3,
                    _ => println!("Invalid input. Please try again."),
                }
            }
            Room::Room1 => {
                println!("You are in Room 1.");
                println!("There is a key on the table.");
                println!("1. Pick up the key");
                println!("2. Leave the key and go back to the starting room");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input.trim() {
                    "1" => {
                        // if the player has the key, they can't pick it up again
                        if has_key {
                            println!("You already have the key.");
                            continue;
                        }
                        println!("You picked up the key.");
                        has_key = true;
                        current_room = Room::Start;
                    }
                    "2" => current_room = Room::Start,
                    _ => println!("Invalid input. Please try again."),
                }
            }
            Room::Room2 => {
                println!("You are in Room 2.");
                println!("There is a locked door in front of you.");
                println!("1. Use the key to unlock the door");
                println!("2. Go back to the starting room");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input.trim() {
                    "1" => {
                        if !has_key {
                            println!("You don't have the key.");
                            continue;
                        }
                        println!("You used the key to unlock the door.");
                        current_room = Room::End;
                    }
                    "2" => current_room = Room::Start,
                    _ => println!("Invalid input. Please try again."),
                }
            }
            Room::Room3 => {
                println!("You are in Room 3.");
                println!("You see a chest in the corner of the room.");
                println!("1. Open the chest");
                println!("2. Go back to the starting room");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input.trim() {
                    "1" => {
                        println!("You opened the chest and found a map.");
                        println!("The map shows a secret passage behind the painting in Room 2.");
                        println!("You go back to the starting room.");
                        current_room = Room::Start;
                    }
                    "2" => current_room = Room::Start,
                    _ => println!("Invalid input. Please try again."),
                }
            }
            Room::End => {
                println!("Congratulations! You have escaped the dungeon.");
                break;
            }
        }
    }
}
