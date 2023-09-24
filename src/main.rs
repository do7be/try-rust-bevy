use bevy::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_JUMP_FORCE: f32 = 44.0;
const GRAVITY: f32 = 9.81 * 100.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Player
    let texture_handle = asset_server.load("images/char.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 2, last: 3 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.33, TimerMode::Repeating)),
        Player {
            direction: Direction::Down,
            grounded: true,
        },
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

#[derive(Component)]
struct Player {
    direction: Direction,
    grounded: bool,
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut player, mut transform, mut velocity) in &mut query {
        // Walk
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction = -1.0;
            transform.scale.x = -1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction = 1.0;
            transform.scale.x = 1.0;
        }

        let new_player_position =
            transform.translation.x + direction * PLAYER_SPEED * time_step.period.as_secs_f32();

        transform.translation.x = new_player_position;

        // Jump
        if !player.grounded {
            velocity.y -= GRAVITY * time_step.period.as_secs_f32();
        } else if keyboard_input.pressed(KeyCode::Space) {
            player.grounded = false;
            velocity.y = PLAYER_JUMP_FORCE * 10.0; // ?
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_systems(Startup, setup)
            .add_systems(Update, animate_sprite)
            .add_systems(
                FixedUpdate,
                (move_player.before(apply_velocity), apply_velocity),
            );
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        .run();
}
