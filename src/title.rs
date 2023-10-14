use try_rust_bevy::consts::*;
use try_rust_bevy::utils::*;

pub mod title_scene {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    pub struct TitlePlugin;

    impl Plugin for TitlePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Title), title_setup)
                .add_systems(Update, control_keys.run_if(in_state(GameState::Title)))
                .add_systems(OnExit(GameState::Title), despawn_screen::<OnTitleScreen>);
        }
    }

    // シーン移動時にコンポーネントを消すためのタグとして使う
    #[derive(Component)]
    struct OnTitleScreen;

    fn title_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/scene_1.png"),
                sprite: Sprite::default(),
                ..default()
            },
            OnTitleScreen,
        ));
    }

    fn control_keys(
        mut game_state: ResMut<NextState<GameState>>,
        keyboard_input: Res<Input<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Z) {
            game_state.set(GameState::Loading);
        }
    }
}
