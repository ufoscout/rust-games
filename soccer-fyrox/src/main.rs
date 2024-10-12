#![allow(clippy::all)]
#![deny(clippy::correctness)]

mod anchor;
mod ball;
mod bare_actor;
mod controls;
mod difficulty;
mod draw_utils;
mod game;
mod game_global;
mod game_hud;
mod game_over_screen;
mod goal;
mod input_controller;
mod math_utils;
mod media;
mod menu_screen;
mod menu_state;
mod my_actor;
mod player;
mod pools;
mod position_utils;
mod rect;
mod state;
mod target;
mod target_handle;
mod team;

pub mod prelude {
    pub use fyrox::{
        core::{
            algebra::{Vector2, Vector3},
            num_traits::Zero,
            pool::{Handle, Pool},
        },
        event::VirtualKeyCode,
        gui::{UiNode, UserInterface},
        scene::{
            base::BaseBuilder,
            dim2::rectangle::{Rectangle, RectangleBuilder},
            graph::Graph,
            node::Node,
            transform::TransformBuilder,
            Scene,
        },
    };
    pub use rand::{thread_rng, Rng};

    pub use crate::anchor::Anchor;
    pub use crate::ball::Ball;
    pub use crate::bare_actor::BareActor;
    pub use crate::controls::Controls;
    pub use crate::difficulty::{Difficulty, DIFFICULTY};
    pub use crate::draw_utils::*;
    pub use crate::game::{Game, DEFAULT_DIFFICULTY};
    pub use crate::game_hud::GameHud;
    pub use crate::game_over_screen::GameOverScreen;
    pub use crate::goal::Goal;
    pub use crate::input_controller::InputController;
    pub use crate::math_utils::*;
    pub use crate::media::{Media, BLANK_IMAGE};
    pub use crate::menu_screen::MenuScreen;
    pub use crate::menu_state::MenuState;
    pub use crate::my_actor::MyActor;
    pub use crate::player::Player;
    pub use crate::pools::Pools;
    pub use crate::position_utils::*;
    pub use crate::rect::Rect;
    pub use crate::state::State;
    pub use crate::target::Target;
    pub use crate::target_handle::TargetHandle;
    pub use crate::team::Team;
    pub use soccer_macros_fyrox::my_actor_based;

    pub const WIDTH: f32 = 800.;
    pub const HEIGHT: f32 = 480.;

    pub const HALF_WINDOW_W: f32 = WIDTH / 2.;

    //# Size of level, including both the pitch and the boundary surrounding it
    pub const LEVEL_W: f32 = 1000.;
    pub const LEVEL_H: f32 = 1400.;
    pub const HALF_LEVEL_W: f32 = LEVEL_W / 2.;
    pub const HALF_LEVEL_H: f32 = LEVEL_H / 2.;

    pub const HALF_PITCH_W: f32 = 442.;
    pub const HALF_PITCH_H: f32 = 622.;

    pub const GOAL_WIDTH: f32 = 186.;
    pub const GOAL_DEPTH: f32 = 20.;
    pub const HALF_GOAL_W: f32 = GOAL_WIDTH / 2.;

    pub const DRIBBLE_DIST_X: f32 = 18.;
    pub const DRIBBLE_DIST_Y: f32 = 16.;

    pub const HUMAN_PLAYER_WITHOUT_BALL_SPEED: f32 = 3.3;

    //DEBUG_SHOW_LEADS = False
    //DEBUG_SHOW_TARGETS = False
    //DEBUG_SHOW_PEERS = False
    //DEBUG_SHOW_SHOOT_TARGET = False
    //DEBUG_SHOW_COSTS = False

    //# Ball physics model parameters
    pub const KICK_STRENGTH: f32 = 11.5;
    pub const DRAG: f32 = 0.98;

    // The below are specific to the port; drawing sequence doesn't work in 3d-based engines; actually,
    // using z-depth is more convenient, since draw calls can happen in any order.
    // The priority on some sprites is based on their coordinates, so we use a min/max.

    pub const CAMERA_NEAR_Z: f32 = -1.0;
    pub const CAMERA_FAR_Z: f32 = 16.0;

    pub const DRAW_MENU_Z: f32 = -1.0;

    pub const DRAW_GAME_HUD_Z: f32 = 0.0;
    pub const DRAW_GAME_SCORES_Z: f32 = -1.0; // need to override the top bar
    pub const DRAW_PITCH_Z: f32 = 16.0;
    pub const DRAW_GOAL_0_Z: f32 = 15.0;
    pub const DRAW_PLAYERS_Z: (f32, f32) = (14.0, 13.0); // includes the ball
    pub const DRAW_SHADOWS_Z: (f32, f32) = (12.0, 11.0); // includes the ball (shadow)
    pub const DRAW_GOAL_1_Z: f32 = 10.0;
    pub const DRAW_ARROWS_Z: f32 = 9.0;

    pub const DRAW_GAME_OVER_BACKGROUND_Z: f32 = 0.0;
    pub const DRAW_GAME_OVER_SCORES_Z: f32 = -1.0;
}

use fyrox::engine::framework::Framework;

use game_global::GameGlobal;

const TITLE: &str = "Substitute Soccer";

fn main() {
    Framework::<GameGlobal>::new().unwrap().title(TITLE).run();
}
