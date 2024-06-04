use std::f64::consts;

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
        victim_body: &RigidBody,
    ) -> Outcome {
        let attacker = attacker.expect("attack always perpetraited");

        let power = 200.0;
        let knockback_per_strength = 5.0;
        let knockback_per_defence = -5.0;
        let knockback_per_damage_taken_squared = 0.015;
        let base_damage_taken_factor = 1.0;

        let hurtbox_vel = hurtbox_body.vel.len();

        let velocity = hurtbox_vel
            + victim.damage_taken
                * (base_damage_taken_factor
                    + stat_factor(attacker.hero.strength_points) * knockback_per_strength
                    + stat_factor(victim.hero.defence_points) * knockback_per_defence)
            + victim.damage_taken.powi(2) * knockback_per_damage_taken_squared
            + power;

        let victim_body_left = V2::new(
            victim_body.pos.x,
            victim_body.pos.y + victim_body.size.y / 2.0,
        );
        let victim_body_right = V2::new(
            victim_body.pos.x + victim_body.size.x,
            victim_body.pos.y + victim_body.size.y / 2.0,
        );
        let hurtbox_body_center_left = V2::new(
            hurtbox_body.pos.x,
            hurtbox_body.pos.y + hurtbox_body.size.y / 2.0,
        );
        let hurtbox_body_center_right = V2::new(
            hurtbox_body.pos.x + hurtbox_body.size.x,
            hurtbox_body.pos.y + hurtbox_body.size.y / 2.0,
        );

        let delta_vel = match self.direction {
            HurtDirection::Left => {
                let extended_hurtbox_body_left =
                    V2::new(victim_body_left.x, hurtbox_body_center_left.y);

                let attack_angle = attack_angle(
                    extended_hurtbox_body_left - hurtbox_body_center_right,
                    victim_body_left - hurtbox_body_center_right,
                );

                let max_angle = consts::PI / 2.0;

                let attack_angle = max(attack_angle, max_angle);

                let attack_angle_percentage = attack_angle / max_angle;
                let y_vel = attack_angle_percentage * velocity;

                if hurtbox_body.pos.y < victim_body.pos.y {
                    V2::new(-velocity, y_vel)
                } else {
                    V2::new(-velocity, -y_vel)
                }
            }
            HurtDirection::Right => {
                let extended_hurtbox_body_right =
                    V2::new(victim_body_right.x, hurtbox_body_center_right.y);

                let attack_angle = attack_angle(
                    extended_hurtbox_body_right - hurtbox_body_center_left,
                    victim_body_right - hurtbox_body_center_left,
                );

                let max_angle = consts::PI / 2.0;

                let attack_angle = max(attack_angle, max_angle);

                let attack_angle_percentage = attack_angle / max_angle;
                let y_vel = attack_angle_percentage * velocity;

                if hurtbox_body.pos.y < victim_body.pos.y {
                    V2::new(velocity, y_vel)
                } else {
                    V2::new(velocity, -y_vel)
                }
            }
            _ => unreachable!(),
        };

        Outcome {
            damage: 10.0,
            delta_vel,
            stun_time: Some(stun_time_by_velocity(0.3, delta_vel)),
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
        victim_body: &RigidBody,
    ) -> Outcome {
        let attacker = attacker.expect("attack always perpetraited");

        let power = 600.0;
        let knockback_per_strength = 5.0;
        let knockback_per_defence = -5.0;
        let knockback_per_damage_taken_squared = 0.015;
        let base_damage_taken_factor = 1.0;

        let hurtbox_vel = hurtbox_body.vel.len();

        let velocity = hurtbox_vel
            + victim.damage_taken
                * (base_damage_taken_factor
                    + stat_factor(attacker.hero.strength_points) * knockback_per_strength
                    + stat_factor(victim.hero.defence_points) * knockback_per_defence)
            + victim.damage_taken.powi(2) * knockback_per_damage_taken_squared
            + power;

        let victim_body_top = V2::new(
            victim_body.pos.x + victim_body.size.x / 2.0,
            victim_body.pos.y,
        );
        let hurtbox_body_center_bottom = V2::new(
            hurtbox_body.pos.x + hurtbox_body.size.x / 2.0,
            hurtbox_body.pos.y + hurtbox_body.size.y,
        );
        let hurtbox_body_center_top = V2::new(
            hurtbox_body.pos.x + hurtbox_body.size.x / 2.0,
            hurtbox_body.pos.y,
        );

        let extended_hurtbox_body_top = V2::new(hurtbox_body_center_top.x, victim_body_top.y);

        let attack_angle = attack_angle(
            extended_hurtbox_body_top - hurtbox_body_center_bottom,
            victim_body_top - hurtbox_body_center_bottom,
        );

        let max_angle = consts::PI / 2.0;

        let attack_angle = max(attack_angle, max_angle);

        let attack_angle_percentage = attack_angle / max_angle;
        let angle_factor = 0.5;
        let x_vel = attack_angle_percentage * velocity * angle_factor;

        let delta_vel = if hurtbox_body.pos.x < victim_body.pos.x {
            V2::new(x_vel, -velocity)
        } else {
            V2::new(-x_vel, -velocity)
        };

        Outcome {
            damage: 10.0,
            delta_vel,
            stun_time: Some(stun_time_by_velocity(0.3, delta_vel)),
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
        _victim_body: &RigidBody,
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
            damage: 5.0,
            delta_vel,
            stun_time: Some(0.5),
        }
    }
}

fn attack_angle(lhs: V2, rhs: V2) -> f64 {
    (lhs.len() / rhs.len()).acos()
}

fn stun_time_by_velocity(stun_time: f64, delta_vel: V2) -> f64 {
    max(stun_time, delta_vel.len() / 2500.0)
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
