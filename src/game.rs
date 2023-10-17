pub mod game_scene {
    use bevy::prelude::*;
    use bevy::sprite::collide_aabb::{collide, Collision};
    use rand::Rng;
    use try_rust_bevy::consts::*;
    use try_rust_bevy::utils::*;

    const FPS: usize = 60;
    const TIME_1F: f32 = 1. / FPS as f32;
    const CHARACTER_SIZE: f32 = 32.;
    const TILE_SIZE: f32 = 32.;
    const PLAYER_JUMP_FORCE: f32 = 44.;
    const PLAYER_WALK_STEP: f32 = 4.;
    const PLAYER_WEAPON_STEP: f32 = 8.;
    const PLAYER_WEAPON_THUNDER_STEP: f32 = 12.;
    const PLAYER_WEAPON_LIFETIME_FOR_SWORD: f32 = 17. * TIME_1F;
    const PLAYER_WEAPON_LIFETIME_FOR_FIRE_ICE: f32 = 30. * TIME_1F;
    const PLAYER_WEAPON_LIFETIME_FOR_THUNDER: f32 = 45. * TIME_1F;
    const GRAVITY: f32 = 9.81;
    const GRAVITY_TIME_STEP: f32 = 0.24; // FPS通りだと重力加速が少ないので経過時間を補正
    const MAP_WIDTH_TILES: u32 = 100;
    const ENEMY_SLIME_WALK_STEP: f32 = 1.;
    const ENEMY_RIZZARD_WALK_STEP: f32 = 4.;

    #[derive(Component)]
    struct OnGameScreen;

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

    #[derive(PartialEq)]
    enum EnemyKind {
        Slime,
        Rizzard,
        RedDeamon,
    }

    #[derive(Component)]
    struct Enemy {
        kind: EnemyKind,
        direction: Direction,
        move_lifetime: usize,
        walk_step: f32,
        stop: bool,
    }

    #[derive(Clone, PartialEq)]
    enum PlayerWeaponKind {
        Sword,
        Fire,
        Ice,
        Thunder,
    }

    #[derive(Component)]
    struct PlayerWeapon {
        kind: PlayerWeaponKind,
        lifetime: Timer,
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
        live: bool,
        jump: bool,
        fall_time: f32,
        jump_start_y: f32,
    }

    #[derive(Resource, Deref, DerefMut)]
    struct ThunderStopTimer(Timer);

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(FixedTime::new_from_secs(TIME_1F)) // 60FPS
                .add_event::<CollisionEvent>()
                .add_systems(OnEnter(GameState::Game), game_setup)
                .add_systems(
                    Update,
                    (animate_sprite, move_camera, die_counter).run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    (check_stage1_clear_system)
                        .run_if(in_state(GameState::Game))
                        .run_if(in_state(StageState::Stage1)),
                )
                .add_systems(
                    FixedUpdate,
                    (
                        control_player_system,
                        check_collision_wall_system.after(control_player_system),
                        check_collision_enemy_system,
                        check_collision_player_weapon_system,
                        move_enemy_system,
                    )
                        .run_if(in_state(GameState::Game)),
                )
                .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
        }
    }

    fn game_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        stage_state: Res<State<StageState>>,
    ) {
        // デスタイマー
        commands.insert_resource(DeathTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        // サンダーを最初だけ一瞬止めるためのタイマー
        commands.insert_resource(ThunderStopTimer(Timer::from_seconds(
            0.0167 * 5., // 5F
            TimerMode::Once,
        )));

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
            OnGameScreen,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(TILE_SIZE * 2., TILE_SIZE * 2., 1.),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.33, TimerMode::Repeating)),
            Player {
                direction: Direction::Right,
                walk: false,
                grounded: true,
                live: true,
                jump: false,
                fall_time: 0.,
                jump_start_y: 0.,
            },
            Character,
            Velocity(Vec2::new(0.0, 0.0)),
        ));

        // Enemy
        let texture_handle = asset_server.load("images/slime.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE),
            2,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 1 };
        commands.spawn((
            OnGameScreen,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: Vec3::new(TILE_SIZE * 12., TILE_SIZE * 2., 0.),
                    scale: Vec3::new(-1., 1., 1.),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.33, TimerMode::Repeating)),
            Character,
            Enemy {
                kind: EnemyKind::Slime,
                direction: Direction::Right,
                move_lifetime: 30, // TODO
                walk_step: ENEMY_SLIME_WALK_STEP,
                stop: false,
            },
        ));
        // TODO: 共通化
        let texture_handle = asset_server.load("images/mohican_lizard.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE),
            2,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 1 };
        commands.spawn((
            OnGameScreen,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: Vec3::new(TILE_SIZE * 6., TILE_SIZE * 11., 0.),
                    scale: Vec3::new(-1., 1., 1.),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.33, TimerMode::Repeating)),
            Character,
            Enemy {
                kind: EnemyKind::Rizzard,
                direction: Direction::Right,
                move_lifetime: 20, // TODO
                walk_step: ENEMY_RIZZARD_WALK_STEP,
                stop: false,
            },
        ));

        let mut map = match stage_state.get() {
            StageState::Stage1 => STAGE1_MAP,
            StageState::Stage2 | StageState::Boss => STAGE2_MAP,
        };
        map.reverse();

        // マップ描画
        for (row, map_str) in map.iter().enumerate() {
            let map_chars = map_str.chars().collect::<Vec<char>>();
            for (column, map_char) in map_chars.iter().enumerate() {
                if *map_char == 'A' || *map_char == 'B' {
                    // Background
                    commands.spawn((
                        OnGameScreen,
                        SpriteBundle {
                            texture: asset_server.load(if *map_char == 'A' {
                                match stage_state.get() {
                                    StageState::Stage1 => "images/map_1.png",
                                    StageState::Stage2 | StageState::Boss => "images/map2_1.png",
                                }
                            } else {
                                match stage_state.get() {
                                    StageState::Stage1 => "images/map_2.png",
                                    StageState::Stage2 | StageState::Boss => "images/map2_2.png",
                                }
                            }),
                            transform: Transform {
                                translation: Vec3::new(
                                    TILE_SIZE * column as f32,
                                    CHARACTER_SIZE * row as f32,
                                    -1.,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
                if *map_char == 'C' {
                    // Wall
                    commands.spawn((
                        OnGameScreen,
                        SpriteBundle {
                            texture: asset_server.load(match stage_state.get() {
                                StageState::Stage1 => "images/map_3.png",
                                StageState::Stage2 | StageState::Boss => "images/map2_3.png",
                            }),
                            transform: Transform {
                                translation: Vec3::new(
                                    TILE_SIZE * column as f32,
                                    CHARACTER_SIZE * row as f32,
                                    0.,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        Wall,
                        Collider,
                    ));
                }
            }
        }
    }

    fn move_camera(
        query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
        mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ) {
        let player_transform = query.single();
        for mut transform in camera_query.iter_mut() {
            transform.translation.x = player_transform
                .translation
                .x
                .max(304.) // 320 - 32 / 2 (タイルの中心が0,0座標なため)
                .min(TILE_SIZE * (MAP_WIDTH_TILES - 11) as f32 - 16.);
            transform.translation.y = 224.; // 240 - 32 / 2
        }
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

    fn check_stage1_clear_system(
        mut game_state: ResMut<NextState<GameState>>,
        mut stage_state: ResMut<NextState<StageState>>,
        mut query: Query<&Transform, With<Player>>,
    ) {
        let transform = query.single_mut();
        if transform.translation.x > TILE_SIZE * (MAP_WIDTH_TILES - 2) as f32 {
            stage_state.set(StageState::Stage2);
            game_state.set(GameState::Loading);
        }
    }

    fn control_player_system(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Player, &mut Transform, &mut Velocity), With<Player>>,
        weapon_query: Query<&PlayerWeapon>,
        mut thunder_timer: ResMut<ThunderStopTimer>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut commands: Commands,
    ) {
        let (mut player, mut transform, mut velocity) = query.single_mut();

        // デス中は何も受け付けない
        if !player.live {
            return;
        }

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
        if player.grounded && keyboard_input.just_pressed(KeyCode::X) {
            player.grounded = false;
            velocity.y = PLAYER_JUMP_FORCE;
            player.jump = true;
            player.jump_start_y = transform.translation.y;
            player.fall_time = 0.;
        }

        // Weapon
        let weapon_kind = if keyboard_input.just_pressed(KeyCode::A) {
            Some(PlayerWeaponKind::Fire)
        } else if keyboard_input.just_pressed(KeyCode::S) {
            Some(PlayerWeaponKind::Ice)
        } else if keyboard_input.just_pressed(KeyCode::D) {
            Some(PlayerWeaponKind::Thunder)
        } else if keyboard_input.just_pressed(KeyCode::Z) {
            Some(PlayerWeaponKind::Sword)
        } else {
            None
        };
        if let Some(weapon_kind) = weapon_kind {
            if weapon_query.iter().any(|weapon| weapon.kind == weapon_kind) {
                // すでに同じ武器を出しているなら何もしない
                return;
            }

            let texture_handle = match weapon_kind {
                PlayerWeaponKind::Fire => asset_server.load("images/fire.png"),
                PlayerWeaponKind::Ice => asset_server.load("images/ice.png"),
                PlayerWeaponKind::Thunder => asset_server.load("images/thunder.png"),
                PlayerWeaponKind::Sword => asset_server.load("images/sword.png"),
            };
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE),
                3,
                1,
                None,
                None,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let animation_indices = AnimationIndices {
                first: 0,
                last: if weapon_kind == PlayerWeaponKind::Thunder {
                    0
                } else {
                    2
                },
            };
            let scale = match player.direction {
                Direction::Right => Vec3::new(1., 1., 0.),
                Direction::Left => Vec3::new(-1., 1., 0.),
            };
            let translation = match weapon_kind {
                PlayerWeaponKind::Thunder => Vec3::new(
                    transform.translation.x,
                    TILE_SIZE * 14.,
                    // 壁よりも手前に表示
                    1.,
                ),
                _ => Vec3::new(
                    match player.direction {
                        Direction::Right => transform.translation.x + TILE_SIZE,
                        Direction::Left => transform.translation.x - TILE_SIZE,
                    },
                    transform.translation.y,
                    // 壁よりも手前に表示
                    1.,
                ),
            };

            commands.spawn((
                OnGameScreen,
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform {
                        translation,
                        scale,
                        ..default()
                    },
                    ..default()
                },
                animation_indices,
                // TODO: 描画フレームは検討の余地あり
                AnimationTimer(Timer::from_seconds(TIME_1F * 6., TimerMode::Repeating)),
                PlayerWeapon {
                    kind: weapon_kind.clone(),
                    lifetime: Timer::from_seconds(
                        match weapon_kind {
                            PlayerWeaponKind::Fire | PlayerWeaponKind::Ice => {
                                PLAYER_WEAPON_LIFETIME_FOR_FIRE_ICE
                            }
                            PlayerWeaponKind::Thunder => PLAYER_WEAPON_LIFETIME_FOR_THUNDER,
                            PlayerWeaponKind::Sword => PLAYER_WEAPON_LIFETIME_FOR_SWORD,
                        },
                        TimerMode::Once,
                    ),
                },
            ));

            // サンダーは最初だけ一瞬止めるのでタイマーをセット
            if weapon_kind == PlayerWeaponKind::Thunder {
                thunder_timer.reset();
            }
        }
    }

    // fn apply_velocity_system(
    //     mut query: Query<
    //         (
    //             &mut Transform,
    //             &mut Velocity,
    //             &mut Player,
    //             &mut AnimationIndices,
    //             &mut TextureAtlasSprite,
    //         ),
    //         With<Player>,
    //     >,
    //     mut timer: ResMut<DeathTimer>,
    // ) {
    //     let (
    //         mut transform,
    //         mut velocity,
    //         mut player,
    //         mut player_animation,
    //         mut player_texture_atlas,
    //     ) = query.single_mut();

    // }

    // プレイヤーの移動先の壁の判定と移動の実施
    #[allow(clippy::type_complexity)]
    fn check_collision_wall_system(
        mut player_query: Query<
            (
                &mut Velocity,
                &mut Transform,
                &mut Player,
                &mut AnimationIndices,
                &mut TextureAtlasSprite,
            ),
            With<Character>,
        >,
        collider_query: Query<
            (Entity, &Transform),
            (With<Collider>, With<Wall>, Without<Character>),
        >,
        mut collision_events: EventWriter<CollisionEvent>,
        mut death_timer: ResMut<DeathTimer>,
    ) {
        let (
            mut player_velocity,
            mut player_transform,
            mut player,
            mut player_animation,
            mut player_texture_atlas,
        ) = player_query.single_mut();
        let player_size = Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE);
        let tile_size = Vec2::new(TILE_SIZE, TILE_SIZE);
        let mut next_time_translation = player_transform.translation;

        // 横移動の判定
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
                for (_collider_entity, transform) in &collider_query {
                    let collision = collide(
                        grounded_translation,
                        player_size,
                        transform.translation,
                        tile_size,
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
                    player.jump = false;
                    player.jump_start_y = player_transform.translation.y;
                    player.fall_time = 0.;
                }
            }
        };

        for (_collider_entity, transform) in &collider_query {
            let collision = collide(
                next_time_translation,
                player_size,
                transform.translation,
                tile_size,
            );
            if let Some(collision) = collision {
                collision_events.send_default();

                match collision {
                    // 左右なら止める
                    Collision::Left | Collision::Right => {
                        next_time_translation.x = player_transform.translation.x;
                    }
                    Collision::Top | Collision::Bottom | Collision::Inside => {}
                }
            }
        }

        // 左右への移動
        if player.walk {
            // 左右移動を反映
            player_transform.translation.x = next_time_translation.x;
            player.walk = false;
        }

        // ジャンプ or 落下していなければこの先の判定をする必要はない
        if player.grounded {
            return;
        }

        player_velocity.y -= GRAVITY * GRAVITY_TIME_STEP;
        player.fall_time += GRAVITY_TIME_STEP;

        let t = player.fall_time;
        next_time_translation.y = if player.jump {
            player.jump_start_y + PLAYER_JUMP_FORCE * t - 0.5 * GRAVITY * t * t
        } else {
            player.jump_start_y - 0.5 * GRAVITY * t * t
        };

        // 縦方向の判定
        let is_fall = player_velocity.y < 0.;
        let is_jump = player_velocity.y > 0.;
        // TODO: collideだとどうしてもジャンプしながら壁にぶつかったときにTOPやBOTTOMが発生しておかしくなるので独自実装に切り替える
        for (_collider_entity, transform) in &collider_query {
            let collision = collide(
                next_time_translation,
                player_size,
                transform.translation,
                tile_size,
            );
            if let Some(collision) = collision {
                collision_events.send_default();

                println!("{}, {:?}", is_jump, collision);
                if is_fall {
                    match collision {
                        // 落ちた先が壁なら下降をやめる
                        Collision::Top | Collision::Inside | Collision::Left | Collision::Right => {
                            player.grounded = true;
                            player_velocity.y = 0.;

                            // めり込まないように位置調整
                            if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                                next_time_translation.y = next_time_translation.y
                                    + (CHARACTER_SIZE - (next_time_translation.y % CHARACTER_SIZE));
                            }
                        }
                        _ => {}
                    }
                }
                if is_jump {
                    match collision {
                        // 壁の下側に頭を当てたら上昇をやめる
                        Collision::Bottom
                        | Collision::Inside
                        | Collision::Left
                        | Collision::Right => {
                            player_velocity.y = 0.;
                            player.jump = false;
                            player.fall_time = 0.;

                            // めり込まないように位置調整
                            if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                                next_time_translation.y = next_time_translation.y
                                    - (next_time_translation.y % CHARACTER_SIZE);
                            }
                            player.jump_start_y = next_time_translation.y
                        }
                        _ => {}
                    }
                }
            }
        }

        // 移動を反映
        player_transform.translation.y = next_time_translation.y;

        // 落ちたときはデス処理
        if player_transform.translation.y < 0. && player.live {
            die(
                &mut player,
                &mut player_transform,
                &mut player_animation,
                &mut player_texture_atlas,
                &mut death_timer,
            );
        }
    }

    #[allow(clippy::type_complexity)]
    fn check_collision_enemy_system(
        mut player_query: Query<
            (
                &mut Transform,
                &mut Player,
                &mut AnimationIndices,
                &mut TextureAtlasSprite,
            ),
            (With<Player>, Without<Enemy>, Without<PlayerWeapon>),
        >,
        enemy_query: Query<(Entity, &Transform), With<Enemy>>,
        mut collision_events: EventWriter<CollisionEvent>,
        mut timer: ResMut<DeathTimer>,
    ) {
        let character_size = Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE);
        let (mut player_transform, mut player, mut player_animation, mut player_texture_atlas) =
            player_query.single_mut();

        // 自分と敵の接触判定
        for (_enemy_entity, enemy_transform) in &enemy_query {
            let collision = collide(
                player_transform.translation,
                character_size,
                enemy_transform.translation,
                character_size,
            );
            if collision.is_some() && player.live {
                collision_events.send_default();
                die(
                    &mut player,
                    &mut player_transform,
                    &mut player_animation,
                    &mut player_texture_atlas,
                    &mut timer,
                );
            }
        }
    }

    // 自分の武器と敵の接触判定
    #[allow(clippy::type_complexity)]
    fn check_collision_player_weapon_system(
        mut commands: Commands,
        mut player_query: Query<
            (&Transform, &Player),
            (With<Player>, Without<Enemy>, Without<PlayerWeapon>),
        >,
        enemy_query: Query<(Entity, &Transform), With<Enemy>>,
        mut player_weapon_query: Query<
            (
                Entity,
                &mut Transform,
                &mut PlayerWeapon,
                &mut AnimationIndices,
            ),
            (With<PlayerWeapon>, Without<Player>, Without<Enemy>),
        >,
        mut collision_events: EventWriter<CollisionEvent>,
        time: Res<Time>,
        mut thunder_timer: ResMut<ThunderStopTimer>,
    ) {
        let character_size = Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE);
        let (player_transform, player) = player_query.single_mut();

        for (
            player_weapon_entity,
            mut player_weapon_transform,
            mut player_weapon,
            mut player_weapon_animation,
        ) in &mut player_weapon_query
        {
            for (enemy_entity, enemy_transform) in &enemy_query {
                let collision = collide(
                    player_weapon_transform.translation,
                    character_size,
                    enemy_transform.translation,
                    character_size,
                );
                if collision.is_some() {
                    collision_events.send_default();
                    // FireとIceなら敵に当たったらdespawnする
                    if player_weapon.kind == PlayerWeaponKind::Fire
                        && player_weapon.kind == PlayerWeaponKind::Ice
                    {
                        commands.entity(player_weapon_entity).despawn();
                    }
                    commands.entity(enemy_entity).despawn();
                }
            }

            // TODO:武器の移動はそれ用のsystemに移動する
            // 武器の移動
            match player_weapon.kind {
                PlayerWeaponKind::Fire => {
                    player_weapon_transform.translation.x += PLAYER_WEAPON_STEP
                        * if player_weapon_transform.scale.x == -1. {
                            -1.
                        } else {
                            1.
                        };
                }
                PlayerWeaponKind::Ice => {
                    player_weapon_transform.translation.x += PLAYER_WEAPON_STEP
                        * if player_weapon_transform.scale.x == -1. {
                            -1.
                        } else {
                            1.
                        };
                    player_weapon_transform.translation.y += PLAYER_WEAPON_STEP;
                }
                PlayerWeaponKind::Thunder => {
                    // サンダーは最初だけ一瞬止める
                    thunder_timer.tick(time.delta());
                    if thunder_timer.finished() {
                        // アニメーション画像を動くものに差し替える
                        if player_weapon_animation.first == 0 {
                            player_weapon_animation.first = 1;
                            player_weapon_animation.last = 2;
                        }
                        player_weapon_transform.translation.y -= PLAYER_WEAPON_THUNDER_STEP;
                    }
                }
                PlayerWeaponKind::Sword => {
                    player_weapon_transform.translation.x = match player.direction {
                        Direction::Right => player_transform.translation.x + CHARACTER_SIZE,
                        Direction::Left => player_transform.translation.x - CHARACTER_SIZE,
                    };
                    player_weapon_transform.translation.y = player_transform.translation.y;
                    player_weapon_transform.scale.x = match player.direction {
                        Direction::Right => 1.,
                        Direction::Left => -1.,
                    };
                }
            }

            player_weapon.lifetime.tick(time.delta());
            if player_weapon.lifetime.finished() {
                commands.entity(player_weapon_entity).despawn();
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn move_enemy_system(
        mut enemy_query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
        wall_query: Query<&Transform, (With<Wall>, Without<Enemy>)>,
        mut collision_events: EventWriter<CollisionEvent>,
    ) {
        for (mut enemy_transform, mut enemy) in &mut enemy_query {
            enemy.move_lifetime -= 1;
            // 現在の行動時間（移動）が終了した時
            if enemy.move_lifetime == 0 {
                enemy.move_lifetime = 30; // TODO

                // 飛ぶ敵か止まっていたら新たな動作の抽選を始める
                // それ以外は行動を継続
                if enemy.kind == EnemyKind::RedDeamon || enemy.stop {
                    enemy.stop = false;
                    let mut rng = rand::thread_rng();
                    // 飛ぶ敵は4方向移動可能
                    let random_max = if enemy.kind == EnemyKind::RedDeamon {
                        4
                    } else {
                        2
                    };
                    let random = rng.gen_range(0..=random_max);

                    match random {
                        // 0ならどちらかに向いて止まる
                        0 => {
                            enemy.direction = if rng.gen() {
                                Direction::Left
                            } else {
                                Direction::Right
                            };
                            enemy.stop = true;
                        }
                        // 右に向く
                        1 => enemy.direction = Direction::Right,
                        // 左に向く
                        2 => enemy.direction = Direction::Left,
                        // 上を向く(飛ぶ敵のみ)
                        3 => enemy.direction = Direction::Right, // TODO
                        // 下を向く(飛ぶ敵のみ)
                        4 => enemy.direction = Direction::Right, // TODO
                        // ランダムをmatchに書くために必要
                        _ => { /* nothing to do */ }
                    };
                    match enemy.direction {
                        Direction::Left => enemy_transform.scale.x = 1.,
                        Direction::Right => enemy_transform.scale.x = -1.,
                    }
                }
            }

            // 敵の移動の判定
            if !enemy.stop {
                // 衝突判定を行うために移動先のtranslationを用意
                // TODO: 上下移動
                let mut next_time_translation = enemy_transform.translation;
                next_time_translation.x = match enemy.direction {
                    Direction::Left => enemy_transform.translation.x - enemy.walk_step,
                    Direction::Right => enemy_transform.translation.x + enemy.walk_step,
                };
                // 壁判定(画面左端)
                if next_time_translation.x <= 0. {
                    enemy.stop = true;
                    // 移動中止
                    next_time_translation = enemy_transform.translation;
                } else {
                    // 壁判定
                    for wall_transform in &wall_query {
                        let collision = collide(
                            next_time_translation,
                            Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE),
                            wall_transform.translation,
                            Vec2::new(TILE_SIZE, TILE_SIZE),
                        );
                        if collision.is_some() {
                            collision_events.send_default();
                            enemy.stop = true;
                            // 移動中止
                            next_time_translation = enemy_transform.translation;
                            break;
                        }
                    }
                }

                // 飛ぶ敵以外は進む先に床がなければ停止させる
                if enemy.kind != EnemyKind::RedDeamon {
                    let mut exist_floor = false;
                    for wall_transform in &wall_query {
                        let mut hoge = next_time_translation;
                        hoge.x = match enemy.direction {
                            Direction::Left => enemy_transform.translation.x - CHARACTER_SIZE,
                            Direction::Right => enemy_transform.translation.x + CHARACTER_SIZE,
                        };
                        hoge.y -= 1.;
                        let collision = collide(
                            hoge,
                            Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE),
                            wall_transform.translation,
                            Vec2::new(TILE_SIZE, TILE_SIZE),
                        );
                        if collision.is_some() {
                            collision_events.send_default();
                            exist_floor = true;
                        }
                    }
                    if !exist_floor {
                        enemy.stop = true;
                        // 移動中止
                        break;
                    }
                }

                // 移動を反映
                enemy_transform.translation = next_time_translation;
                // 向いている方向に画像を向ける
                match enemy.direction {
                    Direction::Left => enemy_transform.scale.x = 1.,
                    Direction::Right => enemy_transform.scale.x = -1.,
                }
            }
        }
        // TODO: 敵の攻撃
    }

    // デス処理
    fn die(
        player: &mut Player,
        transform: &mut Transform,
        animation_indices: &mut AnimationIndices,
        texture_atlas_sprite: &mut TextureAtlasSprite,
        timer: &mut ResMut<DeathTimer>,
    ) {
        player.live = false;
        // デス画像に差し替え
        animation_indices.first = 4;
        animation_indices.last = 4;
        transform.scale.x *= -1.; // デス画像は左右逆になっている
        texture_atlas_sprite.index = 4;
        // デスタイマー起動
        timer.reset();
    }

    #[derive(Resource, Deref, DerefMut)]
    struct DeathTimer(Timer);

    fn die_counter(
        query: Query<&Player, With<Player>>,
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<DeathTimer>,
    ) {
        let player = query.single();
        if !player.live && timer.tick(time.delta()).finished() {
            game_state.set(GameState::Loading);
        }
    }
}
