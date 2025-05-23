// main.rs
use hecs::*;
use macroquad::prelude::*;
mod comps;
mod state;
mod systems;

// Tags
#[derive(Debug, Clone, Copy)]
struct MainCamera;

#[macroquad::main("zoom beez")]
async fn main() {
    // Initialize game state, load textures, setup hecs world
    let mut game = state::Game::new();

    // Main game loop
    loop {
        let delta = get_frame_time();

        // Process systems
        for system in &mut game.systems {
            system.run(&mut game.world, delta);
        }

        // Process events
        for system in &mut game.event_systems {
            system.process_events(&mut game.world, &game.events);
        }
        game.events.clear();

        // Render
        clear_background(BLACK);
        for system in &game.render_systems {
            system.render(&game.world);
        }

        next_frame().await;
    }
}
