pub mod game_scene {
    use bevy::prelude::*;
    use bevy::sprite::collide_aabb::{collide, Collision};
    use rand::Rng;
    use try_rust_bevy::consts::*;
    use try_rust_bevy::utils::*;

    const CHARACTER_SIZE: f32 = 32.;
    const TILE_SIZE: f32 = 32.;
    const PLAYER_JUMP_FORCE: f32 = 44.0;
    const PLAYER_WALK_STEP: f32 = 4.;
    const PLAYER_WEAPON_STEP: f32 = 8.;
    const PLAYER_WEAPON_LIFETIME_FOR_FIRE_ICE: usize = 30;
    const GRAVITY: f32 = 9.81 * 100.0;
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

    #[derive(Clone)]
    enum PlayerWeaponKind {
        Sword,
        Fire,
        Ice,
        Thunder,
    }

    #[derive(Component)]
    struct PlayerWeapon {
        kind: PlayerWeaponKind,
        lifetime: usize,
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
    }
    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
                .add_event::<CollisionEvent>()
                .add_systems(OnEnter(GameState::Game), game_setup)
                .add_systems(
                    Update,
                    (animate_sprite, move_camera, die_counter).run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    FixedUpdate,
                    (
                        check_collision_wall_system
                            .before(apply_velocity_system)
                            .after(control_player_system),
                        control_player_system.before(apply_velocity_system),
                        apply_velocity_system,
                        check_collision_enemy_system,
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
    ) {
        // デスタイマー
        commands.insert_resource(DeathTimer(Timer::from_seconds(2.0, TimerMode::Once)));

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

        let mut map = [
        "CAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACA",
        "CAAAAAAABAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABACA",
        "CAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAACAAAAAAAAABAAAAAAACCCCCCCAAAAAAAAAAAAAAAACA",
        "CAAAAAAAAAAAAAAAAAACACAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAACAAACCAAAAAAAAAAAAAAAAAAAAACCCAAACCCAAACCCA",
        "CBAACCCCCAAAAAAAACCCACCCAAAAACAAAAAAAAAAAAAAAAAAAACCAAAAACCAAAAAAAAAACCCAAAAAAABAAAAAAAAAAAAAAAAAACA",
        "CAAAAAAABAAAAAAAACCCAAAAAAAAACAAAAAAAAAAAAABAAAAAAAAAAAAACAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACA",
        "CACAAAAAAAAACCCCCCCCAABAAAACCCAAACAAAAAAAAAAAAACCAAAAAACACCCAAAAAAAAAAAAAAAAACCCAAAAAAAAAAABAAAAAACA",
        "CACCAAAAAAAAAAAACCCCAAAAAAAAACAAABACABAAAAAAAAAACCAAAAACACAAAACCAAAACCCAAAAAAAAAAAAAAAAAAAAAAAAAAACA",
        "CAACCAAAAAAAAAAACCCCAACCCAAAACAAAAAAACAAAAAAAAAAACCAAAACACAAAAAAAAAAAAAAAABAAAAAAABAAAAAAAAAAAAAAACA",
        "AAAACCCAAAAAABAACCCCAAAAAAAAACAAAAAAABACAAAAAAAAAAAAACCCACAACAAABAAAAAACCCAAAAAAAAAAAAAAAAAAABAAAACA",
        "AAAAAACCCAAAAAAACCCCAABAAAACCCAAAACAAAAAAAAAAABAAAAAAAACACAAAAAAAAAAAAAAAAAAACCCAAAAAAAAAAAAAAAAAACA",
        "AAAAAAAACCCAAAAACCCCAAAACCAAACAAAAAAAAAACAAAAAACCCCCCCCCACCCCCAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAA",
        "AAAAAAAAAAAAAAAACCCCAAAAAAAAACAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAABAACAAAAAABAAAAAAAAAAAAAAAAAAAAAA",
        "CCCCCCCCCCCCCCCCCCCCCCCAAAAAACCCCCCCCCCCCCAACCCCCCCCCCCCCCCCCCCAACAAAACAABACAAAACABAACCCCCCCCCCCCCCC",
        "CCCCCCCCCCCCCCCCCCCCAAAAABAAACCCCCCCCCCCCCAACCCCCCCCCCCCCCCCCCCAACAAAACAAAACAAAACAAAACCCCCCCCCCCCCCC",
    ];
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
                                "images/map_1.png"
                            } else {
                                "images/map_2.png"
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
                            texture: asset_server.load("images/map_3.png"),
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
                .min(TILE_SIZE * (MAP_WIDTH_TILES - 11) as f32); // なぜかずれている
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

    fn control_player_system(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
        weapon_query: Query<&PlayerWeapon>,
        time_step: Res<FixedTime>,
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
        if player.grounded && keyboard_input.pressed(KeyCode::X) {
            player.grounded = false;
            velocity.y = PLAYER_JUMP_FORCE * 9.87; // ?
        }

        // Weapon
        if !weapon_query.is_empty() {
            // すでに武器を出しているなら何もしない
            return;
        }
        let weapon_kind = if keyboard_input.pressed(KeyCode::A) {
            Some(PlayerWeaponKind::Fire)
        } else if keyboard_input.pressed(KeyCode::S) {
            Some(PlayerWeaponKind::Ice)
        } else if keyboard_input.pressed(KeyCode::D) {
            Some(PlayerWeaponKind::Thunder)
        } else if keyboard_input.pressed(KeyCode::Z) {
            Some(PlayerWeaponKind::Sword)
        } else {
            None
        };
        if let Some(weapon_kind) = weapon_kind {
            let texture_handle = match weapon_kind {
                PlayerWeaponKind::Fire => asset_server.load("images/fire.png"),
                PlayerWeaponKind::Ice => asset_server.load("images/ice.png"),
                PlayerWeaponKind::Thunder => asset_server.load("images/thunder.png"),
                PlayerWeaponKind::Sword => asset_server.load("images/sword.png"),
            };
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
            let scale = match player.direction {
                Direction::Right => Vec3::new(1., 1., 0.),
                Direction::Left => Vec3::new(-1., 1., 0.),
            };
            commands.spawn((
                OnGameScreen,
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform::from_xyz(
                        match player.direction {
                            Direction::Right => transform.translation.x + TILE_SIZE,
                            Direction::Left => transform.translation.x - TILE_SIZE,
                        },
                        transform.translation.y,
                        // 壁よりも手前に表示
                        1.,
                    )
                    .with_scale(scale),
                    ..default()
                },
                animation_indices,
                // TODO: 描画フレームは検討の余地あり
                AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                PlayerWeapon {
                    kind: weapon_kind.clone(),
                    lifetime: match weapon_kind {
                        PlayerWeaponKind::Fire | PlayerWeaponKind::Ice => {
                            PLAYER_WEAPON_LIFETIME_FOR_FIRE_ICE
                        }
                        PlayerWeaponKind::Thunder => PLAYER_WEAPON_LIFETIME_FOR_FIRE_ICE, // TODO
                        PlayerWeaponKind::Sword => PLAYER_WEAPON_LIFETIME_FOR_FIRE_ICE,   // TODO
                    },
                },
            ));
        }
    }

    fn apply_velocity_system(
        mut query: Query<
            (
                &mut Transform,
                &mut Velocity,
                &mut Player,
                &mut AnimationIndices,
                &mut TextureAtlasSprite,
            ),
            With<Player>,
        >,
        time_step: Res<FixedTime>,
        mut timer: ResMut<DeathTimer>,
    ) {
        let (
            mut transform,
            mut velocity,
            mut player,
            mut player_animation,
            mut player_texture_atlas,
        ) = query.single_mut();

        // ジャンプ中もしくは落下中なら加速度に重力を作用
        if !player.grounded {
            velocity.y -= GRAVITY * time_step.period.as_secs_f32();
        }

        // 加速度に伴いY軸の位置を変更
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();

        // 落ちたときはデス処理
        if transform.translation.y < 0. && player.live {
            die(
                &mut player,
                &mut transform,
                &mut player_animation,
                &mut player_texture_atlas,
                &mut timer,
            );
        }
    }

    #[allow(clippy::type_complexity)]
    fn check_collision_wall_system(
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

        for (collider_entity, transform, maybe_wall) in &collider_query {
            let collision = collide(
                next_time_translation,
                player_size,
                transform.translation,
                tile_size,
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

        // 縦移動の判定
        if !player.grounded {
            next_time_translation.y += player_velocity.y * time_step.period.as_secs_f32();
        }

        let is_fall = player_velocity.y < 0.;
        let is_jump = player_velocity.y > 0.;
        // TODO: collideだとどうしてもジャンプしながら壁にぶつかったときにTOPやBOTTOMが発生しておかしくなるので独自実装に切り替える
        for (collider_entity, transform, maybe_wall) in &collider_query {
            let collision = collide(
                next_time_translation,
                player_size,
                transform.translation,
                tile_size,
            );
            if let Some(collision) = collision {
                collision_events.send_default();

                if maybe_wall.is_some() {
                    // TODO
                }

                match collision {
                    // 落ちた先が壁なら下降をやめる
                    Collision::Top | Collision::Inside => {
                        if is_fall {
                            player.grounded = true;
                            player_velocity.y = 0.;

                            // めり込まないように位置調整
                            if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                                player_transform.translation.y = if next_time_translation.y > 0. {
                                    next_time_translation.y
                                        + (CHARACTER_SIZE
                                            - (next_time_translation.y % CHARACTER_SIZE))
                                } else {
                                    next_time_translation.y
                                        - (next_time_translation.y % CHARACTER_SIZE)
                                };
                            }
                        }
                    }
                    // 壁の下側に頭を当てたら上昇をやめる
                    Collision::Bottom => {
                        if is_jump {
                            player_velocity.y = 0.;

                            // めり込まないように位置調整
                            if next_time_translation.y % CHARACTER_SIZE != 0.0 {
                                player_transform.translation.y = if next_time_translation.y > 0. {
                                    next_time_translation.y
                                        - (next_time_translation.y % CHARACTER_SIZE)
                                } else {
                                    next_time_translation.y
                                        - (CHARACTER_SIZE
                                            + (next_time_translation.y % CHARACTER_SIZE))
                                };
                            }
                        }
                    }
                    Collision::Left | Collision::Right => {}
                }
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn check_collision_enemy_system(
        mut commands: Commands,
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
        mut player_weapon_query: Query<
            (Entity, &mut Transform, &mut PlayerWeapon),
            (With<PlayerWeapon>, Without<Player>, Without<Enemy>),
        >,
        // TODO
        // mut enemy_weapon_query: Query<(&mut Transform, &mut Player), With<Character>>,
        mut collision_events: EventWriter<CollisionEvent>,
        mut timer: ResMut<DeathTimer>,
    ) {
        let character_size = Vec2::new(CHARACTER_SIZE, CHARACTER_SIZE);
        let (mut player_transform, mut player, mut player_animation, mut player_texture_atlas) =
            player_query.single_mut();

        // TODO: 接触判定の種類ごとにsystemに分割する
        // 自分の武器と敵の接触判定
        for (player_weapon_entity, mut player_weapon_transform, mut player_weapon) in
            &mut player_weapon_query
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
                    // TODO: Thunderならdespawnしない
                    commands.entity(player_weapon_entity).despawn();
                    commands.entity(enemy_entity).despawn();
                }
            }

            // TODO:武器の移動はそれ用のsystemに移動する
            // 武器の移動
            // TODO: Fire以外もつくる
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
                    // TODO
                }
                PlayerWeaponKind::Sword => {
                    // TODO
                }
            }

            player_weapon.lifetime -= 1;
            if player_weapon.lifetime == 0 {
                commands.entity(player_weapon_entity).despawn();
            }
        }

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
