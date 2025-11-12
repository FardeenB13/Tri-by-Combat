use std::io::{self, Write};
use crate::unit::Unit;
use crate::turn::Turn;

/// Prompts the user for input and returns a trimmed string.
pub fn prompt(input: &str) -> String {
    print!("{}", input);
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

/// Prints each unit’s current health and type without taking ownership.
pub fn print_team_status(team_name: &str, team: &Vec<Unit>) {
    println!("=== {} Status ===", team_name);
    for (i, u) in team.iter().enumerate() {
        println!(
            "Slot {}: {} ({}) - HP: {}",
            i + 1,
            u.name,
            u.unit_type.as_str(),
            if u.health > 0 { u.health } else { 0 }
        );
    }
}

/// Counts how many units in a team are alive.
pub fn team_alive_count(team: &Vec<Unit>) -> usize {
    team.iter().filter(|u| u.is_alive()).count()
}

/// Prompts the player to divide their available points between attack and defense.
pub fn get_player_action(unit: &Unit, pool: &mut u32) -> Turn {
    println!(
        "\n{}’s turn. You have {} points to spend.",
        unit.name, *pool
    );

    // Spend points on attack.
    let attack = loop {
        let input = prompt("Points to attack with (0 to skip): ");
        match input.trim().parse::<u32>() {
            Ok(v) if v <= *pool => {
                *pool -= v;
                break v;
            }
            _ => println!("⚠️ Invalid number. Must be between 0 and {}.", *pool),
        }
    };

    // Spend points on defense.
    let defend = if *pool > 0 {
        loop {
            let input = prompt("Points to defend with (0 to skip): ");
            match input.trim().parse::<u32>() {
                Ok(v) if v <= *pool => {
                    *pool -= v;
                    break v;
                }
                _ => println!("⚠️ Invalid number. Must be between 0 and {}.", *pool),
            }
        }
    } else {
        0
    };

    println!("Turn Summary → Attack: {}, Defend: {}", attack, defend);

    Turn { attack, defend }
}
