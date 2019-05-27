use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read, ReadExpect, LazyUpdate},
};

use crate::{
    components::Enemy,
    entities::spawn_explosion,
    resources::SpriteResource,
};


pub struct EnemySystem;
impl<'s> System<'s> for EnemySystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, enemys, mut transforms, time, sprite_resource, lazy_update): Self::SystemData) {
        for (enemy_entity, enemy_component, enemy_transform) in (&*entities, &enemys, &mut transforms).join() {
            enemy_transform.translate_y(-1.0 * enemy_component.speed * time.delta_seconds());

            if enemy_component.health < 0.0 {

                let explosion_position = Vector3::new(
                    enemy_transform.translation()[0], enemy_transform.translation()[1], 0.0,
                );

                spawn_explosion(&entities, &sprite_resource, 3,explosion_position, &lazy_update);
            }

            if enemy_transform.translation()[1] < 0.0 || enemy_component.health < 0.0 {
                let _result = entities.delete(enemy_entity);

            }
        }
    }
}