use bevy::prelude::*;
use crate::components::*;
use crate::game_rules::*;
use crate::resources::Action;

pub fn resolve_attack(
    attacker_pos: IVec2,
    defender_pos: IVec2,
    attack_bonus: i32,
    defender_ac: i32,
    damage: &Damage,
) -> Option<i32> {
    if is_adjacent(attacker_pos, defender_pos) {
        let roll = roll_d20();
        let hit = is_hit(roll, attack_bonus, defender_ac);
        if hit {
            let dmg_roll = roll_damage(damage.num_dice, damage.dice_sides, damage.bonus);
            let crit = is_crit(roll);
            let total_dmg = calculate_damage(dmg_roll, crit);
            println!("Attack: roll {}, hit {}, dmg {}", roll, hit, total_dmg);
            Some(total_dmg)
        } else {
            println!("Miss!");
            None
        }
    } else {
        None
    }
}

pub fn apply_damage(
    mut health: Mut<Health>,
    damage: i32,
) {
    health.0 -= damage;
    if health.0 <= 0 {
        // Could despawn here, but for now just set to 0
        health.0 = 0;
        println!("Entity defeated!");
    }
}

pub fn perform_opportunity_attack(
    commands: &mut Commands,
    attacker_query: &Query<(&Position, &AttackBonus, &Damage, &IsEnemy)>,
    player_pos: IVec2,
    player_ac: i32,
) {
    for (a_pos, attack_bonus, damage, _) in attacker_query.iter() {
        if let Some(dmg) = resolve_attack(a_pos.0, player_pos, attack_bonus.0, player_ac, damage) {
            // Apply to player, but since no mut here, perhaps return dmg and apply elsewhere
            // For simplicity, assume player health is handled in caller
            println!("Opportunity attack damage: {}", dmg);
        }
    }
}
