use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct SemiImplicitEulerConstraint {
    pub target: Entity,
    pub prev_target_pos: Vec3,
    pub current_pos: Vec3,
    pub current_vel: Vec3,
    pub frequency: f32,
    pub damping: f32,
    pub response: f32,
}

impl SemiImplicitEulerConstraint {
    pub fn from_target(target: Entity) -> Self {
        Self {
            target,
            prev_target_pos: Vec3::ZERO,
            current_pos: Vec3::ZERO,
            current_vel: Vec3::ZERO,
            frequency: 1.0,
            damping: 0.5,
            response: 2.0,
        }
    }
}

pub type SieConstraint = SemiImplicitEulerConstraint;

pub fn pos_from_sie_constraints(mut query: Query<(&mut Transform, &SieConstraint)>) {
    for (mut transform, siec) in &mut query {
        transform.translation = siec.current_pos;
    }
}

pub fn update_sie_constraints(
    time: Res<Time>,
    mut siets: Query<&mut SieConstraint, With<Transform>>,
    transforms: Query<&Transform>,
) {
    for mut siet in &mut siets {
        // T
        let delta_time = time.delta_seconds();
        // y, y'
        let (current_pos, current_vel) = (siet.current_pos, siet.current_vel);

        // UPDATE POSITION
        siet.current_pos += current_vel * delta_time;

        // UPDATE VELOCITY

        // x, x'
        let target_pos = transforms.get(siet.target).unwrap().translation;
        let target_vel = {
            if delta_time == 0.0 {
                Vec3::ZERO
            } else {
                let vel_calculated = (target_pos - siet.prev_target_pos) / delta_time;
                siet.prev_target_pos = target_pos;
                vel_calculated
            }
        };

        // Compute constants
        let k1 = siet.damping / (PI * siet.frequency);
        let k2 = 1.0 / ((2.0 * PI * siet.frequency) * (2.0 * PI * siet.frequency));
        let k3 = (siet.response * siet.damping) / (2.0 * PI * siet.frequency);

        let k2_stable = k2.max(1.1 * ((delta_time * delta_time / 4.0) + (delta_time * k1 / 2.0)));

        // Calculate the new velocity
        let accel = (target_pos + (k3 * target_vel) - current_pos - (k1 * current_vel)) / k2_stable;
        siet.current_vel = current_vel + (delta_time * accel);
    }
}
