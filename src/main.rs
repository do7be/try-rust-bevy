use bevy::prelude::*;
use try_rust_bevy::consts::*;

// 各シーン
mod ending;
mod game;
mod loading;
mod stage_title;
mod title;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
            game::game_scene::GamePlugin,
            title::title_scene::TitlePlugin,
            loading::loading_scene::LoadingPlugin,
            stage_title::stage_title_scene::StageTitlePlugin,
            ending::ending_scene::EndingPlugin,
        ))
        .add_state::<GameState>()
        .add_state::<StageState>()
        .add_systems(Startup, setup)
        .run();
}
