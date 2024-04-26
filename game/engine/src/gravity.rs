#[derive(Component, Default, Clone, Debug)]
struct RigidBody {
    pos: (f64, f64),
    vel: (f64, f64),
    rect: (f64, f64),
    gravity: bool,
}
