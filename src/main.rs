use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

const CHARACTER_SIZE: f32 = 32.;
const PLAYER_JUMP_FORCE: f32 = 44.0;
const PLAYER_WALK_STEP: f32 = 4.;
const GRAVITY: f32 = 9.81 * 100.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

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
            direction: Direction::Right,
            walk: false,
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
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-100., CHARACTER_SIZE * -5., 0.),
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
                translation: Vec3::new(0., CHARACTER_SIZE * -4., 0.),
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

enum Direction {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    direction: Direction,
    walk: bool,
    grounded: bool,
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut player, mut transform, mut velocity) in &mut query {
        // Walk
        if keyboard_input.pressed(KeyCode::Left) {
            transform.scale.x = -1.0;
            player.direction = Direction::Left;
            player.walk = true;
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.scale.x = 1.0;
            player.direction = Direction::Right;
            player.walk = true;
        }

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
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut Player), With<Character>>,
    collider_query: Query<
        (Entity, &Transform, Option<&Wall>),
        (With<Collider>, Without<Character>),
    >,
    mut collision_events: EventWriter<CollisionEvent>,
    time_step: Res<FixedTime>,
) {
    let (mut player_velocity, mut player_transform, mut player) = player_query.single_mut();
    let player_size = Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE);

    let mut next_time_translation = player_transform.translation;
    if !player.grounded {
        next_time_translation.y += player_velocity.y * time_step.period.as_secs_f32();
    }

    if player.walk {
        next_time_translation.x = match player.direction {
            Direction::Left => player_transform.translation.x - PLAYER_WALK_STEP,
            Direction::Right => player_transform.translation.x + PLAYER_WALK_STEP,
        };

        // 地面に接しているか検査
        if player.grounded {
            // TODO: 独自実装にすることで全Wallを判定しないようにする
            let mut grounded_translation = player_transform.translation;
            grounded_translation.y -= 1.;
            grounded_translation.x = next_time_translation.x;

            let mut fall_flag = true;
            for (_collider_entity, transform, _maybe_wall) in &collider_query {
                let collision = collide(
                    grounded_translation,
                    player_size,
                    transform.translation,
                    transform.scale.truncate(),
                );

                // 接してる壁があるなら落ちない
                if collision.is_some() {
                    collision_events.send_default();
                    fall_flag = false;
                }
            }
            // 接してる壁がないなら落ちる
            if fall_flag {
                player.grounded = false;
            }
        }
    };

    // TODO: collideだとどうしてもジャンプしながら壁にぶつかったときにTOPやBOTTOMが発生しておかしくなるので独自実装に切り替える
    for (collider_entity, transform, maybe_wall) in &collider_query {
        let collision = collide(
            next_time_translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            collision_events.send_default();

            if maybe_wall.is_some() {
                // TODO
            }

            match collision {
                // 左右なら止める
                Collision::Left | Collision::Right => {
                    next_time_translation.x = player_transform.translation.x;
                }
                // 落ちた先が壁なら下降をやめる
                Collision::Top | Collision::Inside => {
                    player.grounded = true;
                    player_velocity.y = 0.;

                    // めり込まないように位置調整
                    if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                        player_transform.translation.y = if next_time_translation.y > 0. {
                            next_time_translation.y
                                + (CHARACTER_SIZE - (next_time_translation.y % CHARACTER_SIZE))
                        } else {
                            next_time_translation.y - (next_time_translation.y % CHARACTER_SIZE)
                        };
                    }
                }
                // 壁の下側に頭を当てたら上昇をやめる
                Collision::Bottom => {
                    player_velocity.y = 0.;

                    // めり込まないように位置調整
                    if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                        player_transform.translation.y = if next_time_translation.y > 0. {
                            next_time_translation.y - (next_time_translation.y % CHARACTER_SIZE)
                        } else {
                            next_time_translation.y
                                - (CHARACTER_SIZE + (next_time_translation.y % CHARACTER_SIZE))
                        };
                    }
                }
            }
        }
    }

    // 左右への移動
    if player.walk {
        // 左右移動を反映
        player_transform.translation.x = next_time_translation.x;
        player.walk = false;
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
