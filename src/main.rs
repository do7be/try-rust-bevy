use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

const CHARACTER_SIZE: f32 = 32.;
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
struct Character;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Event, Default)]
struct CollisionEvent;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Player
    let texture_handle = asset_server.load("images/char.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE),
        5,
        1,
        None,
        None,
    );
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
        Character,
        Velocity(Vec2::new(0.0, 0.0)),
    ));

    // Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(50., CHARACTER_SIZE, 0.),
                scale: Vec3::new(100., 32., 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(100., CHARACTER_SIZE * 4.0, 0.),
                scale: Vec3::new(100., 32., 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., CHARACTER_SIZE * -2., 0.),
                scale: Vec3::new(100., 32., 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-100., CHARACTER_SIZE * 5., 0.),
                scale: Vec3::new(100., 32., 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider,
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

#[allow(clippy::type_complexity)]
fn check_for_collisions(
    mut commands: Commands,
    mut character_query: Query<
        (&mut Velocity, &mut Transform, Option<&mut Player>),
        With<Character>,
    >,
    collider_query: Query<
        (Entity, &Transform, Option<&Wall>),
        (With<Collider>, Without<Character>),
    >,
    mut collision_events: EventWriter<CollisionEvent>,
    time_step: Res<FixedTime>,
) {
    let (mut character_velocity, mut character_transform, mut maybe_player) =
        character_query.single_mut();
    let character_size = if maybe_player.is_some() {
        Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE)
    } else {
        character_transform.scale.truncate()
    };

    for (collider_entity, transform, maybe_wall) in &collider_query {
        let mut next_time_translation = character_transform.translation;
        if let Some(ref player) = maybe_player {
            if !player.grounded {
                next_time_translation.y += character_velocity.y * time_step.period.as_secs_f32();
            }
        }

        let collision = collide(
            next_time_translation,
            character_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            collision_events.send_default();

            if maybe_wall.is_some() {
                // TODO
            }

            match collision {
                // TODO: 左右なら止める処理を入れる
                Collision::Left => { /* TODO */ }
                Collision::Right => { /* TODO */ }
                // TODO: 上にぶつかったときにワープしないようにするs
                Collision::Top | Collision::Bottom | Collision::Inside => {
                    if let Some(ref mut player) = maybe_player {
                        character_velocity.y = 0.;
                        player.grounded = true;

                        if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                            character_transform.translation.y = if next_time_translation.y > 0. {
                                next_time_translation.y
                                    + (CHARACTER_SIZE - (next_time_translation.y % CHARACTER_SIZE))
                            } else {
                                next_time_translation.y - (next_time_translation.y % CHARACTER_SIZE)
                            };
                        }
                    }
                }
            }
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, animate_sprite)
            .add_systems(
                FixedUpdate,
                (
                    check_for_collisions
                        .before(apply_velocity)
                        .after(move_player),
                    move_player.before(apply_velocity),
                    apply_velocity,
                ),
            );
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (640.0, 480.0).into(),
                        title: "Innocent Heart".into(),
                        ..default()
                    }),
                    ..default()
                }),
            GamePlugin,
        ))
        .run();
}
