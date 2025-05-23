// Game state
use crate::systems::*;
use hecs::World;

pub struct Game {
    pub world: World,
    pub events: Vec<GameEvent>,
    pub systems: Vec<Box<dyn GameSystem>>,
    pub render_systems: Vec<Box<dyn RenderSystem>>,
    pub event_systems: Vec<Box<dyn EventSystem>>,
}

impl Game {
    pub fn new() -> Self {
        unimplemented!();
    }
}
