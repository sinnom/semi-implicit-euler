//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_position)
        .add_system(velocity_from_siet.after(update_position))
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct SemiImplicitEulerTracking {
    target: Entity,
    frequency: f32,
    damping: f32,
    response: f32,
}

type Siet = SemiImplicitEulerTracking;

fn update_position(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += **velocity;
    }
}

fn velocity_from_siet(mut query: Query<(&mut Velocity, &Transform, &Siet)>) {
    for (velocity, transform, siet) in &query {
        let Ok(pos_in) = query.get_component::<Transform>(siet.target) else {
          panic!("SemiImplicitEulerTracking component has a target, but the target does not a Transform.");
        };
        let Ok(vel_in)  = query.get_component::<Velocity>(siet.target) else {
          panic!("Siet component has a target, but the target lacks a Velocity.");
        };
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.2)));
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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
