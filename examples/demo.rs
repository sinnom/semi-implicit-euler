//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent, RngPlugin};
use semi_implicit_euler::{SemiImplicitEulerPlugin, SieConstraint};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let mut app = App::new();

    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: format!("debug,wgpu_core=warn,wgpu_hal=warn,{CRATE_NAME}=debug"),
    }));

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    }));

    app.add_plugin(RngPlugin::default())
        .add_plugin(SemiImplicitEulerPlugin)
        .add_startup_system(setup)
        // TODO: Add a system set label and order this to come before
        // or after the constraint calculations
        .add_system(random_pos)
        .run();
}

#[derive(Component)]
struct RandomTeleport {
    timer: Timer,
}
fn random_pos(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut RngComponent, &mut RandomTeleport)>,
) {
    const RANDOM_SCALE: f32 = 3.0;
    for (mut transform, mut rng, mut rand_teleport) in &mut query {
        rand_teleport.timer.tick(time.delta());

        if rand_teleport.timer.just_finished() {
            transform.translation =
                Vec3::new(rng.f32() * RANDOM_SCALE, 0.0, rng.f32() * RANDOM_SCALE)
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let target = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.2,
                subdivisions: 2,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RandomTeleport {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        })
        .insert(RngComponent::from(&mut global_rng))
        .id();

    // SIETracking cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.2, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(SieConstraint::from_target(target));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
