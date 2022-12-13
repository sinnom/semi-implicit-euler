//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_turborand::{rng::Rng, DelegatedRng, GlobalRng, RngComponent, RngPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RngPlugin::default())
        .add_startup_system(setup)
        .add_system(position_from_velocity)
        .add_system(velocity_from_siet.after(position_from_velocity))
        .add_system(random_pos)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct SemiImplicitEulerTracking {
    target: Entity,
    prev_target_pos: Vec3,
    frequency: f32,
    damping: f32,
    response: f32,
}

impl SemiImplicitEulerTracking {
    fn from_target(target: Entity) -> Self {
        Self {
            target,
            prev_target_pos: Vec3::ZERO,
            frequency: 1.0,
            damping: 0.5,
            response: 2.0,
        }
    }
}

type Siet = SemiImplicitEulerTracking;

fn position_from_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += **velocity * time.delta_seconds();
    }
}

fn velocity_from_siet(
    time: Res<Time>,
    mut siets: Query<(&mut Siet, Entity), (With<Velocity>, With<Transform>)>,
    mut velocities: Query<&mut Velocity>,
    transforms: Query<&Transform>,
) {
    for (mut siet, siet_entity) in &mut siets {
        let delta_time = time.delta_seconds();
        // y, y'
        let current_pos = transforms.get(siet_entity).unwrap().translation;
        let current_vel = **velocities.get(siet_entity).unwrap();

        // x, x'
        let target_pos = transforms.get(siet.target).unwrap().translation;
        let target_vel = {
            // Get velocity from a component if its there
            if let Ok(vel_component) = velocities.get(siet.target) {
                vel_component.0
            }
            // Or calculate the velocity from previous position on previous frames
            else {
                let vel_calculated = (target_pos - siet.prev_target_pos) / delta_time;
                siet.prev_target_pos = target_pos;
                vel_calculated
            }
        };

        // Compute constants
        let k1 = siet.damping / (PI * siet.frequency);
        let k2 = 1.0 / ((2.0 * PI * siet.frequency) * (2.0 * PI * siet.frequency));
        let k3 = siet.response * siet.damping / (2.0 * PI * siet.frequency);

        let k2_stable = k2.max(1.1 * ((delta_time * delta_time / 4.0) + (delta_time * k1 / 2.0)));

        // Calculate the new velocity
        let accel = (target_pos + (k3 * target_vel) - current_pos - (k1 * current_vel)) / k2_stable;
        let new_vel = current_vel + time.delta_seconds() * accel;

        let mut velocity = velocities.get_mut(siet_entity).unwrap();
        *velocity = Velocity(new_vel);
    }
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
                radius: 0.7,
                subdivisions: 2,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Velocity(Vec3::new(0.0, 1.0, 0.0)))
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
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(Siet::from_target(target));
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
