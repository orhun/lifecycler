use std::f32::consts::PI;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tank_system);
}

#[derive(Component)]
pub struct Tank;

fn setup_tank_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Tank,
        SceneBundle {
            transform: Transform::from_scale(Vec3::new(2., 0.5, 2.))
                .with_rotation(Quat::from_rotation_x(PI / 2.)),
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank.glb")),
            ..default()
        },
    ));
    commands.spawn((SceneBundle {
        transform: Transform::from_scale(Vec3::new(1.8, 1., 0.5))
            .with_translation(Vec3::new(0., -1.75, 0.))
            .with_rotation(Quat::from_rotation_x(0.)),
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("gravel.glb")),
        ..default()
    },));
}
