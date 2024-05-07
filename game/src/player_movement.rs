use engine::{query, rigid_body::RigidBody, Collider, Component, System};

#[derive(Component)]
pub struct PlayerMovement;

pub struct PlayerMovementSystem(pub u64);
impl System for PlayerMovementSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerMovement, RigidBody, Collider) {
            let d_down = ctx.key_pressed(engine::Keycode::D);
            let a_down = ctx.key_pressed(engine::Keycode::A);
            let w_down = ctx.key_pressed(engine::Keycode::W);
            let collider = ctx.select::<Collider>(id).clone();
            let body = ctx.select::<RigidBody>(id);
            body.vel.0 = if d_down && !a_down {
                400.0
            } else if !d_down && a_down {
                -400.0
            } else {
                0.0
            };
            if collider
                .colliding
                .is_some_and(|dir| dir.facing(engine::collision::Direction::Bottom))
                && w_down
            {
                body.vel.1 = -800.0;
            }
        }
        Ok(())
    }
}
