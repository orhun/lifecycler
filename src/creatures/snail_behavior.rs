use std::f32::consts::PI;

use bevy::{ecs::query::QueryEntityError, prelude::*};

use crate::general::AttemptDespawn;

use super::behavior::{CreatureBehavior, CreatureOperations, CreatureRng};

pub struct SnailOperations<'a> {
    transform: &'a mut Transform,
    behavior: &'a mut CreatureBehavior,
}

impl<'a> SnailOperations<'a> {
    pub(super) fn new(transform: &'a mut Transform, behavior: &'a mut CreatureBehavior) -> Self {
        Self {
            transform,
            behavior,
        }
    }
}

impl<'a> CreatureOperations for SnailOperations<'a> {
    fn behavior_debut(&mut self, time: &Time, _rng: &mut CreatureRng) {
        self.transform.translation.x += time.delta_seconds() * Self::base_speed();

        if self.transform.translation.x > -1. {
            self.start_swim_right();
        }
    }

    fn behavior_idle(&mut self, time: &Time) {
        self.transform.translation.y += time.elapsed_seconds().sin() / 1000.;
    }

    fn behavior_seek_pellet(
        &mut self,
        time: &Time,
        rng: &mut CreatureRng,
        pellet: Result<(Entity, &Transform), QueryEntityError>,
        commands: &mut Commands,
    ) {
        if let Ok((pellet_entity, pellet_transform)) = pellet {
            if self.transform.translation.x < pellet_transform.translation.x {
                self.face_right();
            } else {
                self.face_left();
            }

            let (min, max) = Self::valid_area();
            self.transform.translation = self.transform.translation.move_towards(
                pellet_transform.translation.clamp(min, max),
                time.delta_seconds() * Self::base_speed() * 2.,
            );

            if self
                .transform
                .translation
                .distance(pellet_transform.translation)
                < 0.1
            {
                if let Some(mut entity) = commands.get_entity(pellet_entity) {
                    entity.insert(AttemptDespawn);
                }
            }
        } else {
            self.start_seek_point(rng);
        }
    }

    fn face_right(&mut self) {
        *self.transform =
            self.transform
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 3. * PI / 2., 0.));
    }

    fn face_left(&mut self) {
        *self.transform =
            self.transform
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., PI / 2., 0.));
    }

    fn rank_pellet(&mut self, pellet_transform: &Transform) -> f32 {
        self.transform()
            .translation
            .xz()
            .distance(pellet_transform.translation.xy())
    }

    fn check_pellet(&mut self, _rank: f32) -> bool {
        true
    }

    fn base_speed() -> f32 {
        0.05
    }

    fn valid_area() -> (Vec3, Vec3) {
        (Vec3::new(-1.5, -1.7, -0.4), Vec3::new(1.5, -1.7, 0.4))
    }

    fn behavior(&mut self) -> &mut CreatureBehavior {
        self.behavior
    }

    fn transform(&mut self) -> &mut Transform {
        self.transform
    }

    fn valid_point_buffer() -> Vec3 {
        Vec3::new(0.2, 0., 0.)
    }
}
