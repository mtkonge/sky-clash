use crate::{
    hurtbox::{HurtDirection, HurtboxProfile, Outcome},
    player::Player,
};
use engine::{max, rigid_body::RigidBody, V2};

pub enum AttackKind {
    Up,
    Down,
    Left,
    Right,
}

pub struct SideAttackProfile {
    pub direction: HurtDirection,
}

impl HurtboxProfile for SideAttackProfile {
    fn outcome(
        &self,
        victim: &Player,
        attacker: Option<&Player>,
        hurtbox_body: &RigidBody,
    ) -> Outcome {
        let attacker = attacker.expect("attack always perpetraited");

        let power = 200.0;
        let knockback_per_strength = 100.0;
        let knockback_per_defence = -100.0;
        let knockback_per_damage_taken_squared = 0.015;
        let base_damage_taken_squared = 1.0;

        let hurtbox_vel = hurtbox_body.vel.len();

        let velocity = hurtbox_vel
            + (base_damage_taken_squared
                + stat_factor(attacker.hero.strength_points) * knockback_per_strength
                + stat_factor(victim.hero.defence_points) * knockback_per_defence)
                * victim.damage_taken.powi(2)
                * knockback_per_damage_taken_squared
            + power;

        let delta_vel = match self.direction {
            HurtDirection::Left => V2::new(-velocity, 0.0),
            HurtDirection::Right => V2::new(velocity, 0.0),
            _ => unreachable!(),
        };

        Outcome {
            damage: 10.0,
            delta_vel,
            stun_time: Some(0.3),
        }
    }
}

pub struct UpAttackProfile;
impl HurtboxProfile for UpAttackProfile {
    fn outcome(
        &self,
        victim: &Player,
        attacker: Option<&Player>,
        hurtbox_body: &RigidBody,
    ) -> Outcome {
        let attacker = attacker.expect("attack always perpetraited");

        let power = 600.0;
        let knockback_per_strength = 100.0;
        let knockback_per_defence = -100.0;
        let knockback_per_damage_taken_squared = 0.015;
        let base_damage_taken_squared = 1.0;

        let hurtbox_vel = hurtbox_body.vel.len();

        let delta_vel = V2::new(
            0.0,
            -(hurtbox_vel
                + (base_damage_taken_squared
                    + stat_factor(attacker.hero.strength_points) * knockback_per_strength
                    + stat_factor(victim.hero.defence_points) * knockback_per_defence)
                    * victim.damage_taken.powi(2)
                    * knockback_per_damage_taken_squared
                + power),
        );

        Outcome {
            damage: 10.0,
            delta_vel,
            stun_time: Some(max(0.3, delta_vel.len() / 2500.0)),
        }
    }
}

pub struct DownAttackProfile;
impl HurtboxProfile for DownAttackProfile {
    fn outcome(
        &self,
        victim: &Player,
        _attacker: Option<&Player>,
        hurtbox_body: &RigidBody,
    ) -> Outcome {
        let power = 600.0;
        let knockback_per_damage_taken_squared = 0.0025;

        let hurtbox_vel = hurtbox_body.vel.len();

        let delta_vel = V2::new(
            0.0,
            -(hurtbox_vel
                + victim.damage_taken.powi(2) * knockback_per_damage_taken_squared
                + power),
        );
        Outcome {
            damage: 10.0,
            delta_vel,
            stun_time: Some(0.5),
        }
    }
}

fn strength_and_defence_modifier(victim_defence: i64, owner_strength: i64) -> f64 {
    let base_strength = 1.0;
    let base_defence = 1.0;
    base_strength + stat_factor(owner_strength) - (stat_factor(victim_defence) + base_defence)
}

fn stat_factor(stat: i64) -> f64 {
    let stat_max = 24.0;
    stat as f64 / stat_max
}
