use bevy::prelude::*;

use crate::{components::*, constants::*, resources::*, utils::random_pipe_position};

// simple system that makes the Press Space Bar text blink
pub fn blink_space_bar_text(time: Res<Time>, mut query: Query<(&mut PressSpaceBarText, &mut Visibility)>,) {

    let (mut space, mut visibility) = query.single_mut();
 
    let timer = &mut space.0;
    timer.tick(time.delta());

    if timer.finished() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }

}

// simple system that moves the background to the left
pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut background_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 20. * delta;
 
    background_transform.translation.x -= delta_x;

    // Reset the background position once it moves out of the screen
    if background_transform.translation.x < -BACKGROUNG_IMAGE_WIDTH {
        background_transform.translation.x = 0.;
    }
}

// move the ground to the left
pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut ground_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta; // move faster than background because it's closer to the camera perspective
 
    ground_transform.translation.x -= delta_x;
 
    if ground_transform.translation.x < -BACKGROUNG_IMAGE_WIDTH {
        ground_transform.translation.x = 0.;
    }
}


pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut TextureAtlas)>) {
    for (mut bird, mut texture_atlas) in query.iter_mut() {
        let delta = time.delta();
 
        bird.timer.tick(delta);
 
        if bird.timer.finished() {
            texture_atlas.index = if texture_atlas.index == 2 {
                0
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

// simple system that starts the game when the space bar is pressed
//
// Using the Without<PressSpaceBarText> in the game_over_query is critical, 
// because in the previous query we have queried for PressSpaceBarText and Visibility, 
// so we need to exclude the PressSpaceBarText from this query. 
// If we don't use the Without<PrestSpaceBarText> to exclude the PressSpaceBarText from this query, 
// Bevy will panic, if you want to learn more about this error, read https://bevyengine.org/learn/errors/b0001/
pub fn start_game(
    mut game: ResMut<Game>,
    mut space_query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PressSpaceBarText>)>,
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut upper_pipe_query: Query<(&mut Transform, &mut UpperPipe), (With<UpperPipe>, Without<Bird>)>,
    mut lower_pipe_query: Query<
        &mut Transform,
        (With<LowerPipe>, Without<Bird>, Without<UpperPipe>),
    >,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
 
    // Reset the pipes
    if game.state == GameState::GameOver {
        for (i, (mut transform, mut upper_pipe)) in upper_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 200.0 + 200.;
 
            upper_pipe.passed = false;
            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }
 
        for (i, mut transform) in lower_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 200.0 + 200.;
 
            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }
    };

    game.state = GameState::Active;
 
    // Reset the bird position
    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity = 0.0;
        transform.translation.y = 0.0;
        transform.rotation = Quat::from_rotation_z(0.0);
    }

    // Hiding the PressSpaceBarText
    let (mut space, mut visibility) = space_query.single_mut();
    space.0.reset();
    *visibility = Visibility::Hidden;
 
    // Hiding the GameOverText
    let mut game_over_visibility = game_over_query.single_mut();
    *game_over_visibility = Visibility::Hidden;
}

// simple system that makes the bird fall down
pub fn gravity(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut query: Query<(&mut Bird, &mut Transform)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (mut bird, mut transform) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        let gravity = 9.8;

        let delta_v = gravity * 150. * delta;
        let delta_y = bird.velocity * delta;
        
        bird.velocity -= delta_v;
        transform.translation.y = (transform.translation.y + delta_y).min(260.0);
 
        // Rotate the bird
        let rotation = bird.velocity / 600.0;
        let max_rotation = 0.5;
        transform.rotation = Quat::from_rotation_z(rotation.max(-max_rotation).min(max_rotation));
 
        // Check if the bird hits the ground
        let ground_y = -250.0;
        let ground_height = 112.0;
        let bird_height = 24.0;
 
        let collision_point = ground_y + ground_height / 2.0 + bird_height / 2.0;
 
        if transform.translation.y < collision_point {
            transform.translation.y = collision_point;
            bird.velocity = 0.0;
 
            game.state = GameState::GameOver;
            *game_over_query.single_mut() = Visibility::Visible;
 
            // play game over sound
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/hit.ogg"),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        }
    }
}

// simple system that makes the bird jump
pub fn jump(
    mut query: Query<&mut Bird>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
 
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/wing.ogg"),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
 
    for mut bird in query.iter_mut() {
        bird.velocity = 400.0;
    }
}

// simple system that moves the pipes to the left and check for bird collision
pub fn pipes(
    time: Res<Time>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &mut Transform)>,
    mut lower_pipe_query: Query<(&LowerPipe, &mut Transform), Without<UpperPipe>>,
    mut bird_query: Query<&Transform, (With<Bird>, Without<LowerPipe>, Without<UpperPipe>)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta;
 
    let utmost_right_pipe = upper_pipe_query
        .iter()
        .max_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap())
        .unwrap()
        .1
        .translation
        .x;
 
    let new_pipe_position = utmost_right_pipe + 200.0;
    let (lower_y, upper_y) = random_pipe_position();
    let out_of_screen_x = (-WINDOW_WIDTH / 2.) - 26.;
 
    for (mut upper_pipe, mut transform) in upper_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;
 
        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = upper_y;
            upper_pipe.passed = false;
        }
    }
 
    for (_, mut transform) in lower_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;
 
        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = lower_y;
        }
    }
 
    let is_collision = |bird_transform: &Transform, pipe_transform: &Transform| -> bool {
        let bird_x = bird_transform.translation.x;
        let bird_y = bird_transform.translation.y;
        let bird_width = 34.0;
        let bird_height = 24.0;
 
        let pipe_x = pipe_transform.translation.x;
        let pipe_y = pipe_transform.translation.y;
        let pipe_width = 52.0;
        let pipe_height = 320.0;
 
        let collision_x = bird_x + bird_width / 2.0 > pipe_x - pipe_width / 2.0
            && bird_x - bird_width / 2.0 < pipe_x + pipe_width / 2.0;
        let collision_y = bird_y + bird_height / 2.0 > pipe_y - pipe_height / 2.0
            && bird_y - bird_height / 2.0 < pipe_y + pipe_height / 2.0;
 
        collision_x && collision_y
    };
 
    for bird_transform in bird_query.iter_mut() {
        let mut game_over = || {
            game.state = GameState::GameOver;
            *game_over_query.single_mut() = Visibility::Visible;
 
            // Play game over sound
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/hit.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });
        };
 
        for (_, transform) in upper_pipe_query.iter_mut() {
            if is_collision(bird_transform, &transform) {
                game_over();
            }
        }
 
        for (_, transform) in lower_pipe_query.iter_mut() {
            if is_collision(bird_transform, &transform) {
                game_over();
            }
        }
    }
}

// simple system that scores the game
pub fn score(
    mut game: ResMut<Game>,
    bird_query: Query<(&Bird, &Transform)>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (_, bird_transform) in bird_query.iter() {
        for (mut upper_pipe, transform) in upper_pipe_query.iter_mut() {
            let passed = transform.translation.x < bird_transform.translation.x;
            let passed_state = upper_pipe.passed;
 
            if passed && !passed_state {
                game.score += 1;
                upper_pipe.passed = true;
 
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/point.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                });
 
                println!("Score: {}", game.score);
            }
        }
    }
}

pub fn render_score(game: Res<Game>, mut query: Query<&mut TextureAtlas, With<ScoreText>>) {
    let score_string = format!("{:03}", game.score); // Ensure at least 3 digits, pad with zeros
    let score_digits: Vec<usize> = score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
 
    for (digit, mut texture_atlas) in score_digits.iter().zip(query.iter_mut()) {
        texture_atlas.index = *digit;
    }
}