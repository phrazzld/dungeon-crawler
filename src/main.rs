use colored::*;
use std::io::Write;
use std::{io, thread, time};

fn main() {
    match intro() {
        true => {
            start_crawling();
        }
        false => {}
    }
}

fn intro() -> bool {
    let intro_banner = "
▓█████▄  █    ██  ███▄    █   ▄████ ▓█████  ▒█████   ███▄    █     ▄████▄   ██▀███   ▄▄▄       █     █░ ██▓    ▓█████  ██▀███
▒██▀ ██▌ ██  ▓██▒ ██ ▀█   █  ██▒ ▀█▒▓█   ▀ ▒██▒  ██▒ ██ ▀█   █    ▒██▀ ▀█  ▓██ ▒ ██▒▒████▄    ▓█░ █ ░█░▓██▒    ▓█   ▀ ▓██ ▒ ██▒
░██   █▌▓██  ▒██░▓██  ▀█ ██▒▒██░▄▄▄░▒███   ▒██░  ██▒▓██  ▀█ ██▒   ▒▓█    ▄ ▓██ ░▄█ ▒▒██  ▀█▄  ▒█░ █ ░█ ▒██░    ▒███   ▓██ ░▄█ ▒
░▓█▄   ▌▓▓█  ░██░▓██▒  ▐▌██▒░▓█  ██▓▒▓█  ▄ ▒██   ██░▓██▒  ▐▌██▒   ▒▓▓▄ ▄██▒▒██▀▀█▄  ░██▄▄▄▄██ ░█░ █ ░█ ▒██░    ▒▓█  ▄ ▒██▀▀█▄
░▒████▓ ▒▒█████▓ ▒██░   ▓██░░▒▓███▀▒░▒████▒░ ████▓▒░▒██░   ▓██░   ▒ ▓███▀ ░░██▓ ▒██▒ ▓█   ▓██▒░░██▒██▓ ░██████▒░▒████▒░██▓ ▒██▒
▒▒▓  ▒ ░▒▓▒ ▒ ▒ ░ ▒░   ▒ ▒  ░▒   ▒ ░░ ▒░ ░░ ▒░▒░▒░ ░ ▒░   ▒ ▒    ░ ░▒ ▒  ░░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░ ▓░▒ ▒  ░ ▒░▓  ░░░ ▒░ ░░ ▒▓ ░▒▓░
░ ▒  ▒ ░░▒░ ░ ░ ░ ░░   ░ ▒░  ░   ░  ░ ░  ░  ░ ▒ ▒░ ░ ░░   ░ ▒░     ░  ▒     ░▒ ░ ▒░  ▒   ▒▒ ░  ▒ ░ ░  ░ ░ ▒  ░ ░ ░  ░  ░▒ ░ ▒░
░ ░  ░  ░░░ ░ ░    ░   ░ ░ ░ ░   ░    ░   ░ ░ ░ ▒     ░   ░ ░    ░          ░░   ░   ░   ▒     ░   ░    ░ ░      ░     ░░   ░
░       ░              ░       ░    ░  ░    ░ ░           ░    ░ ░         ░           ░  ░    ░        ░  ░   ░  ░   ░
░                                                                ░
";

    // Introduce the player to the scene
    println!("{}", intro_banner.green());
    println!("Are you ready to play?");

    let mut player_ready = String::new();
    io::stdin()
        .read_line(&mut player_ready)
        .expect("Failed to read line");
    player_ready = player_ready.trim().to_lowercase();

    let yes_strings = [
        "yes",
        "y",
        "1",
        "ok",
        "alright",
        "indeed",
        "sure",
        "sure thing",
        "yup",
        "why not",
        "absolutely",
    ];

    if yes_strings.contains(&player_ready.as_str()) {
        println!("Let's begin...");
        true
    } else {
        println!("Maybe next time...");
        false
    }
}

fn dramatic_print(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().expect("Something went wrong.");
        match c {
            '.' | '\n' => thread::sleep(time::Duration::from_millis(300)),
            ':' => thread::sleep(time::Duration::from_millis(200)),
            ',' => thread::sleep(time::Duration::from_millis(100)),
            _ => thread::sleep(time::Duration::from_millis(25))
        }
    }
    print!("\n");
}

fn start_crawling() {
    let open_forest = "\nIt is night. You are in an open forest. The moon is full, and the sky is bright with stars.
Ahead of you the forest gets thicker and darker.
To your left is a hill, with a large house sitting atop it.
To your right is more of the same open forest.

What do you do?
A: Go ahead, deeper into the forest.
B: Go left, to the house on the hill.
C: Go right, exploring the edge of the forest.
D: Go back, turn around and head home.";

    dramatic_print(open_forest);

    loop {
        let mut player_choice = String::new();
        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        player_choice = player_choice.trim().to_lowercase();

        match player_choice.as_str() {
            "a" => {
                deeper_into_the_forest();
                break;
            },
            "b" => {
                to_the_house_on_the_hill();
                break;
            },
            "c" => {
                explore_the_woods_edge();
                break;
            },
            "d" => {
                go_home();
                break;
            },
            _ => {
                println!("\n");
                println!("{}", "Please answer A, B, C, or D".red());
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    }
}

fn deeper_into_the_forest() {
    println!("STUB");
}

fn to_the_house_on_the_hill() {
    println!("STUB");
}

fn explore_the_woods_edge() {
    println!("STUB");
}

fn go_home() {
    println!("STUB");
}
