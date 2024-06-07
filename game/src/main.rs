#![allow(dead_code)]

use engine::spawn;
use server::Server;
use sound_player::sound_player;

mod attacks;
mod backend_connection;
mod game;
mod hero_creator;
mod hero_info;
mod hud;
mod hurtbox;
mod keyset;
mod knockoff;
mod main_menu;
mod mock_connection;
mod player;
mod player_interaction;
mod server;
mod sound_player;
mod sprite_renderer;
mod start_game;
mod timer;
mod ui_components;

// pub const FONT: &str = "assets/ttf/OpenSans.ttf";
pub const FONT: &str = "assets/ttf/Jaro-Regular.ttf";

fn main() {
    let mut connection = backend_connection::BackendConnection::new();
    // let connection = mock_connection::MockConnection::new();
    let mut server = Server::new(connection.clone());

    let (mut sound_player, sound_player_join_handle) = sound_player();
    sound_player.set_music_volume(0.3);

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        spawn!(&mut ctx, sound_player.clone());
        spawn!(&mut ctx, server.clone());
        ctx.add_system(main_menu::MainMenuSystem);

        game.run();
        server.quit();
        sound_player.quit();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        connection.run().await;
    });

    let _ = game_thread.join();
    let _ = sound_player_join_handle.join();
}

#[test]
pub fn test_v2() {
    use engine::V2;

    assert_eq!(
        V2::new(3.0, 3.0).move_along(V2::new(3.0, 0.0), 20.0),
        V2::new(3.0 + 20.0, 3.0)
    );
    assert_eq!(
        V2::new(3.0, 0.0).extend_distance(4.0),
        V2::new(3.0 + 4.0, 0.0)
    );
}

#[test]
pub fn test_rects_within_reach() {
    use engine::physics::*;
    use engine::V2;
    assert!(Rect::from_f64(0.0, 0.0, 10.0, 0.0)
        .moving(V2::new(10.0, 10.0))
        .rect_within_reach(Rect::from_f64(15.0, 0.0, 10.0, 10.0)));
    assert!(!Rect::from_f64(0.0, 0.0, 10.0, 0.0)
        .moving(V2::new(10.0, 10.0))
        .rect_within_reach(Rect::from_f64(40.0, 0.0, 10.0, 10.0)));
}

#[test]
pub fn test_point_vec_line_segment_intersect() {
    use engine::physics::*;
    use engine::V2;

    macro_rules! named {
        ($name: ident) => {
            (stringify!($name), $name)
        };
    }
    let check_a = {
        let edge_a = (V2::new(10.0, 10.0), V2::new(40.0, 10.0));
        let line_a = (V2::new(20.0, 0.0), V2::new(10.0, 20.0));
        let line_b = (V2::new(25.0, 0.0), V2::new(0.0, 25.0));
        let line_c = (V2::new(30.0, 0.0), V2::new(-10.0, 20.0));
        let intersection = V2::new(25.0, 10.0);

        [named!(line_a), named!(line_b), named!(line_c)]
            .into_iter()
            .map(|line| (line, named!(edge_a), intersection))
            .collect::<Vec<_>>()
    };
    let check_b = {
        let edge_b = (V2::new(40.0, 40.0), V2::new(40.0, 10.0));
        let line_d = (V2::new(50.0, 20.0), V2::new(-20.0, 10.0));
        let line_e = (V2::new(50.0, 25.0), V2::new(-25.0, 0.0));
        let line_f = (V2::new(50.0, 30.0), V2::new(-20.0, -10.0));
        let intersection = V2::new(40.0, 25.0);

        [named!(line_d), named!(line_e), named!(line_f)]
            .into_iter()
            .map(|line| (line, named!(edge_b), intersection))
            .collect::<Vec<_>>()
    };
    let check_c = {
        let edge_c = (V2::new(40.0, 40.0), V2::new(10.0, 40.0));
        let line_i = (V2::new(20.0, 50.0), V2::new(10.0, -20.0));
        let line_h = (V2::new(25.0, 50.0), V2::new(0.0, -25.0));
        let line_g = (V2::new(30.0, 50.0), V2::new(-10.0, -20.0));
        let intersection = V2::new(25.0, 40.0);

        [named!(line_i), named!(line_h), named!(line_g)]
            .into_iter()
            .map(|line| (line, named!(edge_c), intersection))
            .collect::<Vec<_>>()
    };
    let check_d = {
        let edge_d = (V2::new(10.0, 10.0), V2::new(10.0, 40.0));
        let line_d = (V2::new(0.0, 20.0), V2::new(20.0, 10.0));
        let line_e = (V2::new(0.0, 25.0), V2::new(25.0, 0.0));
        let line_f = (V2::new(0.0, 30.0), V2::new(20.0, -10.0));
        let intersection = V2::new(10.0, 25.0);

        [named!(line_d), named!(line_e), named!(line_f)]
            .into_iter()
            .map(|line| (line, named!(edge_d), intersection))
            .collect::<Vec<_>>()
    };
    [check_a, check_b, check_c, check_d]
        .into_iter()
        .flatten()
        .for_each(
            |(
                (line_name, (pos, delta_pos)),
                (edge_name, (edge_p0, edge_p1)),
                expected_intersection,
            )| {
                let intersection =
                    pos.moving(delta_pos).line_segment_intersect(Line::new(edge_p0, edge_p1))
                        .map(|Intersection{pos, distance_factor: _ }| pos);

                assert!(
                    intersection.is_some(),
                    "expected line {line_name} to intersect with edge {edge_name}, got None"
                );

                let intersection = intersection.expect("we asserted it to be Some");
                assert_eq!(intersection, expected_intersection, "expected line {line_name} to intersect with edge {edge_name} at {expected_intersection:?}, got {intersection:?}")
            },
        );
}
