use hecs::World;
// Events
#[derive(Debug)]
pub enum GameEvent {
    PlayerMoved(f32),
    AttackerSpawned,
}

// Systems traits
pub trait GameSystem {
    fn run(&mut self, world: &mut World, delta: f32);
}

pub trait RenderSystem {
    fn render(&self, world: &World);
}

pub trait EventSystem {
    fn process_events(&mut self, world: &mut World, events: &[GameEvent]);
}

// Systems structs
pub struct PlayerInputSystem;
pub struct AttackerAISystem;
pub struct MovementSystem;
pub struct ParallaxSystem;
pub struct RenderingSystem;
pub struct EventHandlerSystem;
