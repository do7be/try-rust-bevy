use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, With};

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
