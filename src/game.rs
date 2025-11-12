use crate::{
    enemy_turn::enemy_turn,
    misc::{get_player_action, print_team_status, prompt, team_alive_count},
    turn::Turn,
    unit::{Unit, UnitType},
};
use rand::Rng;

/// Returns base points per round, gradually increasing until capped at 4.
fn base_points_for_round(round: u32) -> u32 {
    let p = 1 + round;
    if p >= 4 { 4 } else { p }
}

/// Calculates attack, defense, and resulting damage for both sides.
fn resolve_pair(
    player_unit: &mut Unit,
    player_turn: &Turn,
    enemy_unit: &mut Unit,
    enemy_turn: &Turn,
    log: &mut Vec<String>,
) {
    let p_att = player_turn.attack as i32;
    let p_def = player_turn.defend as i32;
    let e_att = enemy_turn.attack as i32;
    let e_def = enemy_turn.defend as i32;

    // Player attack resolution
    if p_att > e_def {
        let dmg = player_unit.base_damage * (p_att - e_def);
        enemy_unit.health -= dmg;
        log.push(format!(
            "{} attacks {} (atk {} vs def {}) -> {} dmg",
            player_unit.name, enemy_unit.name, p_att, e_def, dmg
        ));
    } else if p_att > 0 {
        log.push(format!(
            "{}'s attack ({} pts) was blocked.",
            player_unit.name, p_att
        ));
    }

    // Enemy attack resolution
    if e_att > p_def {
        let dmg = enemy_unit.base_damage * (e_att - p_def);
        player_unit.health -= dmg;
        log.push(format!(
            "{} attacks {} (atk {} vs def {}) -> {} dmg",
            enemy_unit.name, player_unit.name, e_att, p_def, dmg
        ));
    } else if e_att > 0 {
        log.push(format!(
            "{}'s attack ({} pts) was blocked.",
            enemy_unit.name, e_att
        ));
    }
}

/// Main tournament loop — handles team creation, enemy generation, and match progression
pub fn run_game() {
    println!("Tournament Starting...\n");
    println!("Instructions:");
    println!(" - You will choose 3 units for your team.");
    println!(" - Each round, face off against an enemy team of 3 units.");
    println!(" - Allocate points to attack, defend, or save each turn.");
    println!(" - Base points increase each round, up to a maximum of 4.");
    println!(" - Points carry over if unspent, up to a maximum of 8.");
    println!(" - Defeat all enemy units to win the tournament!\n");
    println!("Good luck!\n");
    println!("Choose 3 units for your team.");
    println!("Large = High Health, Low Damage");
    println!("Medium = Balanced");
    println!("Light = Low Health, High Damage\n");

    // === PLAYER TEAM CREATION ===
    let mut player_team: Vec<Unit> = Vec::new();

    // Prompt player to choose 3 units by type and name
    for i in 1..=3 {
        // Ask for unit type until valid input
        let unit_type = loop {
            let choice = prompt(&format!("Unit {} type (1=Large, 2=Medium, 3=Light): ", i));
            match choice.trim() {
                "1" => break UnitType::Large,
                "2" => break UnitType::Medium,
                "3" => break UnitType::Light,
                _ => println!("⚠️ Invalid choice. Enter 1, 2, or 3."),
            }
        };

        // Ask for non-empty unit name.
        let name = loop {
            let name_input = prompt("Enter unit name: ");
            if name_input.trim().is_empty() {
                println!("⚠️ Name cannot be empty. Try again.");
            } else {
                break name_input.trim().to_string();
            }
        };

        player_team.push(Unit::new(&name, unit_type));
    }

    // Display initial team stats.
    print_team_status("Player", &player_team);

    // === ENEMY TEAM GENERATION ===
    let mut rng = rand::thread_rng();
    let mut enemies: Vec<Vec<Unit>> = Vec::new();

    // Predefined names/themes for each enemy team (3 rounds total)
    let team_themes = ["The Raptors", "The Sentinels", "The Elite"];

    // Create 3 themed enemy teams with random unit compositions
    for (round_idx, theme_name) in team_themes.iter().enumerate() {
        let mut team: Vec<Unit> = Vec::new();
        println!(
            "\nGenerating enemy team for Round {} — {}!",
            round_idx + 1,
            theme_name
        );

        for slot in 1..=3 {
            let utype = match rng.gen_range(0..3) {
                0 => UnitType::Large,
                1 => UnitType::Medium,
                _ => UnitType::Light,
            };
            let name = format!("{}_{}", theme_name.replace(' ', "_"), slot);
            let u = Unit::new(&name, utype);
            team.push(u);
        }
        enemies.push(team);
    }

    // === TOURNAMENT PROGRESSION ===
    let mut saved_points: u32 = 0;

    // Each enemy team represents one tournament round
    for (round_idx, enemy_team) in enemies.iter_mut().enumerate() {
        println!("\n== Round {} ==", round_idx + 1);
        print_team_status("Enemy", &enemy_team);

        let mut round = 1;
        let mut enemy_saved_points = vec![0; 3];

        // Loop continues until one side’s team is wiped out
        while team_alive_count(&player_team) > 0 && team_alive_count(&enemy_team) > 0 {
            let base_points = base_points_for_round(round);
            let mut pool = std::cmp::min(8, base_points + saved_points);

            println!("\nRound {} - You have {} points.", round, pool);
            print_team_status("Player", &player_team);
            print_team_status("Enemy", &enemy_team);

            // Auto-swap frontliner if they are KO'd.
            if !player_team[0].is_alive() {
                if let Some(next_idx) = player_team.iter().position(|u| u.is_alive()) {
                    player_team.swap(0, next_idx);
                    println!(
                        "⚠️ {} has fallen! Swapping in {} as the new frontliner.",
                        player_team[next_idx].name, player_team[0].name
                    );
                } else {
                    println!("💀 All your units have fallen!");
                    return;
                }
            }

            // Allow player to manually swap the frontliner if alive
            println!(
                "\nCurrent frontliner: {} ({} HP)",
                player_team[0].name, player_team[0].health
            );
            println!("Would you like to swap?");
            println!("1. Keep current");
            println!("2. Swap with unit 2 (if alive)");
            println!("3. Swap with unit 3 (if alive)");

            // Handle swap input and validation
            loop {
                let input = prompt("Choose option (1–3): ");
                match input.trim().parse::<usize>() {
                    Ok(1) => break,
                    Ok(2) if player_team.len() >= 2 && player_team[1].is_alive() => {
                        player_team.swap(0, 1);
                        println!("Swapped {} to the front!", player_team[0].name);
                        break;
                    }
                    Ok(3) if player_team.len() >= 3 && player_team[2].is_alive() => {
                        player_team.swap(0, 2);
                        println!("Swapped {} to the front!", player_team[0].name);
                        break;
                    }
                    _ => println!("⚠️ Invalid choice or target is dead."),
                }
            }

            // Player frontliner chooses attack/defend/save allocation
            let acting_unit = &player_team[0];
            let action = get_player_action(acting_unit, &mut pool); 
            saved_points = pool.min(8); // carry over unspent points

            // Enemy selects frontliner and generates its move
            let enemy_acting_index = match enemy_team.iter().position(|u| u.is_alive()) {
                Some(idx) => idx,
                None => {
                    println!("All enemies defeated!");
                    break;
                }
            };
            let base_points = base_points_for_round(round);
            let  enemy_pool =
                std::cmp::min(8, base_points + enemy_saved_points[enemy_acting_index]);

            let enemy_action = enemy_turn(
                enemy_saved_points[enemy_acting_index],
                enemy_pool,
                &enemy_team[enemy_acting_index],
            );

            // Save any leftover points (if your enemy_turn supports it)
            enemy_saved_points[enemy_acting_index] =
                (enemy_pool.saturating_sub(enemy_action.attack + enemy_action.defend)).min(8);

            println!(
                "\n⚔️  {} (Player) vs {} (Enemy)",
                player_team[0].name, enemy_team[enemy_acting_index].name
            );

            // Resolve both units actions and log the outcome
            let mut log = Vec::new();
            resolve_pair(
                &mut player_team[0],
                &action,
                &mut enemy_team[enemy_acting_index],
                &enemy_action,
                &mut log,
            );

            // Print battle results for this round
            println!("\nRound Log:");
            for l in log {
                println!(" - {}", l);
            }

            // Show updated team health/status
            print_team_status("Player", &player_team);
            print_team_status("Enemy", &enemy_team);

            // End-of-round win/loss checks
            if team_alive_count(&enemy_team) == 0 {
                println!("🏆 Enemy team defeated!");

                // Heal player team to full for next round
                for unit in &mut player_team {
                    let (max_hp, _) = unit.unit_type.stats();
                    unit.health = max_hp as i32;
                }
                println!("💖 Your team has recovered to full health!");
                break;
            } else if team_alive_count(&player_team) == 0 {
                println!("💀 Your team was defeated.");
                return;
            }

            round += 1; // Increment round counter
        }
    }

    println!("\nTournament complete!");
}
