use bevy::prelude::*;

use crate::{components::*, constants::{BACKGROUNG_IMAGE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, utils::random_pipe_position};
 
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/background.png"),
            sprite: Sprite {
                // Setting the size of the sprite to the window size, so it covers the whole screen.
                // As the texture is smaller than the window, it will be scaled to fit the window size.
                // The type of scaling is defined in the ImageScaleMode, if not specified, it defaults to Stretch.
                // We set a custom size to the sprite that is larger than the window size, so we move the background to the left
                custom_size: Some(Vec2::new(WINDOW_WIDTH + BACKGROUNG_IMAGE_WIDTH * 2., WINDOW_HEIGHT)), // Adding a custom size
                ..default() // Everything else is set to default
            },
            ..default()
        },
        // Tiled repeats the texture to fill the whole screen
        ImageScaleMode::Tiled {
            tile_x: true, // Only repeat on the x-axis
            tile_y: false, // no repeat on the y-axis
            stretch_value: 1., // no stretching
        },
        // This is a marker component, so we can easily query the background entity
        Background
    ));

    // Spawn the Ground
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/base.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + BACKGROUNG_IMAGE_WIDTH * 2., 112.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -250., 1.),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.,
        },
        Ground,
    ));

        // Game Over Text
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/game-over.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                visibility: Visibility::Hidden,
                ..default()
            },
            GameOverText,
        ));


        // Press Space Bar Text
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/space.png"),
                transform: Transform::from_xyz(0.0, -50.0, 1.0),
                ..default()
            },
            PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
        ));

        // Score Text
        {
            let number_layout: TextureAtlasLayout =
                TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
            let number_texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(number_layout);
        
            // We spawn 3 numbers to represent a maximum score of 999
            for i in 0..3 {
                let x_starting_point = -350. + (i as f32 * (24. + 2.)); // 24 is the width + 2 is the space between the numbers
         
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("texture/numbers.png"),
                        transform: Transform::from_xyz(x_starting_point, 200., 1.),
                        ..default()
                    },
                    TextureAtlas {
                        index: 0,
                        layout: number_texture_atlas_layout.clone(),
                    },
                    ScoreText,
                ));
            }
        }

            // Spawn the bird
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/bird.png"),
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        TextureAtlas {
            index: 1,
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(34, 24),
                3,
                1,
                None,
                None,
            )),
        },
        Bird {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            velocity: 0.,
        },
    ));

    // Spawn the pipes
    for i in 0..5 {
        let delta_x = i as f32 * 200.;
        let (lower_y, upper_y) = random_pipe_position();
        let mut transform = Transform::from_xyz(350. + delta_x, lower_y, 0.5);
     
        // Spawn Lower Pipe
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/pipe.png"),
                transform,
                ..default()
            },
            LowerPipe,
        ));
     
        // Rotating the upper pipe
        transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
        // Changing the y position of the upper pipe
        transform.translation.y = upper_y;
     
        // Spawn Upper Pipe
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/pipe.png"),
                transform,
                ..default()
            },
            UpperPipe {
                passed: false,
            },
        ));
    }
    

}