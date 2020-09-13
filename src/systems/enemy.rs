use crate::constants::ARENA_HEIGHT;
use crate::{
    audio::{play_sfx, Sounds},
    components::{
        choose_random_name, Defense, Enemy, EnemyType, Hitbox2DComponent, Motion2DComponent,
        Rigidbody,
    },
    constants::{ARENA_MIN_Y, EXPLOSION_Z},
    entities::{spawn_consumable, spawn_explosion},
    resources::{ConsumableEntityData, SpriteResource},
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
};
use std::collections::HashMap;

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, Hitbox2DComponent>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        ReadExpect<'s, HashMap<String, ConsumableEntityData>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut enemys,
            mut defenses,
            mut transforms,
            mut motions,
            hitboxes,
            time,
            sprite_resource,
            lazy_update,
            storage,
            sounds,
            audio_output,
            consumable_pool,
        ): Self::SystemData,
    ) {
        for (enemy_entity, enemy_component, enemy_transform, enemy_motion, enemy_hitbox) in (
            &*entities,
            &mut enemys,
            &mut transforms,
            &mut motions,
            &hitboxes,
        )
            .join()
        {
            //constrain in arena
            enemy_component.constrain_to_arena(enemy_transform, enemy_motion, enemy_hitbox);

            //transform the spaceship in x and y by the currrent velocity in x and y
            enemy_component.update_position(enemy_transform, time.delta_seconds(), enemy_motion);

            enemy_component.health -= enemy_component.poison;

            //conditions for despawning
            if enemy_transform.translation()[1] + enemy_hitbox.height / 2.0 < ARENA_MIN_Y {
                //defense is damage is enemy gets past
                for defense in (&mut defenses).join() {
                    defense.defense -= enemy_component.defense_damage;
                }
                entities
                    .delete(enemy_entity)
                    .expect("unable to delete entity");
            } else if enemy_component.health < 0.0 {
                //enemy us deleted, explosion is spawned and item dropped
                let death_position = Vector3::new(
                    enemy_transform.translation()[0],
                    enemy_transform.translation()[1],
                    EXPLOSION_Z,
                );

                entities
                    .delete(enemy_entity)
                    .expect("unable to delete entity");

                spawn_explosion(
                    &entities,
                    &sprite_resource,
                    enemy_component.explosion_sprite_idx,
                    death_position,
                    &lazy_update,
                );

                play_sfx(&sounds.explosion_sfx, &storage, audio_output.as_deref());

                let name = choose_random_name(&enemy_component.collectables_probs);
                if !name.is_empty() {
                    spawn_consumable(
                        &entities,
                        &sprite_resource,
                        consumable_pool[name].clone(),
                        death_position,
                        &lazy_update,
                    );
                }
            }

            //behavior for enemies based on its enemy_type attribute
            match enemy_component.enemy_type {
                EnemyType::Pawn => {
                    //accelerate in -y direction
                    enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Drone => {
                    //accelerate in -y direction
                    enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Hauler => {
                    //accelerate in -y direction
                    enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Strafer => {
                    //accelerate in -y direction
                    enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    enemy_component.accelerate(1.0, 0.0, enemy_motion);
                }

                EnemyType::RepeaterBody => {
                    //accelerate in -y direction
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }
                }

                EnemyType::RepeaterHead => {
                    //accelerate in -y direction
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }
                }

                EnemyType::RepeaterShoulder => {
                    //accelerate in -y direction
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }

                    //rotate back and forth
                    if enemy_transform.euler_angles().2 > 0.1 {
                        enemy_motion.angular_velocity = 0.05;
                    } else if enemy_transform.euler_angles().2 < -0.1 {
                        enemy_motion.angular_velocity = -0.05;
                    }
                }

                EnemyType::RepeaterArm => {
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }
                }
            }
        }
    }
}
