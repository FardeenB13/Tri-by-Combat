use crate::turn::Turn;
use crate::unit::Unit;
use rand::Rng;

/// Determines how an enemy allocates its points each turn.
/// Returns a `Turn` struct (attack, defend, save).
pub fn enemy_turn(saved_points: u32, base_points: u32, unit: &Unit) -> Turn {
    let mut rng = rand::thread_rng();
    let max_pool = (saved_points + base_points).min(8);
    let mut pool = max_pool;

    let mut attack = 0;
    let mut defend = 0;

    // Get health ratio for smarter behavior
    let max_hp = unit.unit_type.stats().0 as f32;
    let hp_ratio = unit.health as f32 / max_hp;

    // --- Strategy selection ---
    if hp_ratio < 0.33 {
        // Low HP → mostly defend
        defend = rng.gen_range(pool / 2..=pool);
        pool -= defend;
        if pool > 0 && rng.gen_bool(0.3) {
            attack = rng.gen_range(1..=pool);
            pool -= attack;
        }
    } else if hp_ratio > 0.66 {
        // High HP → mostly attack
        attack = rng.gen_range(pool / 2..=pool);
        pool -= attack;
        if pool > 0 && rng.gen_bool(0.4) {
            defend = rng.gen_range(1..=pool);
            pool -= defend;
        }
    } else {
        // Balanced HP → mix randomly
        attack = rng.gen_range(0..=pool);
        pool -= attack;
        if pool > 0 {
            defend = rng.gen_range(0..=pool);
            pool -= defend;
        }
    }

    let _save = pool; // whatever remains

    Turn { attack, defend }
}
