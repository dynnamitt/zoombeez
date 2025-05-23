use macroquad::texture::Texture2D;
// Components
#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct Sprite {
    texture: Texture2D,
    scale: f32,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
struct Attacker {
    speed: f32,
    attack_range: f32,
}

#[derive(Debug, Clone)]
struct ParallaxBackground {
    layers: [Texture2D; 2],
    scroll_speeds: [f32; 2],
    positions: [f32; 2],
}
