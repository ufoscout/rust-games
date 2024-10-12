use bevy::prelude::*;
 
#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressSpaceBarText(pub Timer); // timer will repeat every 0.5 seconds

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Bird {
    pub timer: Timer,
    pub velocity: f32,
}

#[derive(Component)]
pub struct UpperPipe{
    pub passed: bool,
}
 
#[derive(Component)]
pub struct LowerPipe;