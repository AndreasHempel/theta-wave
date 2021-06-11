use crate::{
    audio::Sounds,
    components::{
        AbilityDirection, BarrelRollAbilityComponent, BarrierComponent, HealthComponent,
        Motion2DComponent,
    },
    entities::EffectType,
    events::{ConsumableGetEvent, ItemGetEvent, PlayAudioEvent, PlayerCollisionEvent},
    resources::{GameParametersResource, SpriteSheetsResource},
    spawnable::{
        components::{BlastComponent, ConsumableComponent, ItemComponent, MobComponent},
        entities::spawn_effect,
        resources::EffectsResource,
    },
    systems::{barrier_collision, immovable_collision, standard_collision},
    weapons::BlastType,
};
use amethyst::{
    core::transform::Transform,
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct SpaceshipMobCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipMobCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Read<'s, GameParametersResource>,
        ReadStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        ReadStorage<'s, BarrelRollAbilityComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            game_parameters,
            mobs,
            mut motions,
            mut healths,
            barrel_roll_abilities,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an mob entity?
            if let Some(mob) = mobs.get(event.colliding_entity) {
                let spaceship_motion = motions.get_mut(event.player_entity).unwrap();
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();

                let collision_damage_immune = if let Some(barrel_roll_ability) =
                    barrel_roll_abilities.get(event.player_entity)
                {
                    if let AbilityDirection::None = barrel_roll_ability.action_direction {
                        false
                    } else {
                        barrel_roll_ability.steel_barrel
                    }
                } else {
                    false
                };

                if !collision_damage_immune {
                    spaceship_health.take_damage(mob.collision_damage);
                }

                if let Some(collision_velocity) = event.collision_velocity {
                    if event.collider_immovable {
                        immovable_collision(
                            spaceship_motion,
                            collision_velocity,
                            game_parameters.min_collision_knockback,
                        );
                    } else {
                        standard_collision(
                            spaceship_motion,
                            collision_velocity,
                            game_parameters.min_collision_knockback,
                        );
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipBlastCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipBlastCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, BarrelRollAbilityComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            entities,
            mut healths,
            mut blasts,
            barrel_roll_abilities,
            transforms,
            effects_resource,
            sprite_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with a blast component?
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                let player_hittable = if let Some(barrel_roll_ability) =
                    barrel_roll_abilities.get(event.player_entity)
                {
                    if let AbilityDirection::None = barrel_roll_ability.action_direction {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };

                // first check if the blast is allied with the player
                // TODO blast should not hit if player is currently barrel rolling
                if player_hittable {
                    match blast.blast_type {
                        // using match here for ease of adding enemy blast effects (such as poison) in the future
                        BlastType::Enemy => {
                            entities
                                .delete(event.colliding_entity)
                                .expect("unable to delete entity");

                            spawn_effect(
                                &EffectType::EnemyBlastExplosion,
                                blast_transform.clone(),
                                &effects_resource,
                                &sprite_resource,
                                &entities,
                                &lazy_update,
                            );
                            spaceship_health.take_damage(blast.damage);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipItemCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipItemCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Entities<'s>,
        ReadStorage<'s, ItemComponent>,
        Write<'s, EventChannel<ItemGetEvent>>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            entities,
            items,
            mut item_get_event_channel,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an item component?
            if let Some(item) = items.get(event.colliding_entity) {
                item_get_event_channel.single_write(ItemGetEvent {
                    player_entity: event.player_entity,
                    item_type: item.item_type.clone(),
                });

                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["shotgun_cock"].clone(),
                });

                entities
                    .delete(event.colliding_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipConsumableCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipConsumableCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Entities<'s>,
        ReadStorage<'s, ConsumableComponent>,
        Write<'s, EventChannel<ConsumableGetEvent>>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            entities,
            consumables,
            mut consumable_get_event_channel,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with a consumable entity?
            if let Some(consumable) = consumables.get(event.colliding_entity) {
                consumable_get_event_channel.single_write(ConsumableGetEvent {
                    player_entity: event.player_entity,
                    consumable_type: consumable.consumable_type.clone(),
                });

                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects[&consumable.sound_effect].clone(),
                });

                entities
                    .delete(event.colliding_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipArenaBorderCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipArenaBorderCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        ReadStorage<'s, BarrierComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            barriers,
            mut motion_2ds,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with a barrier?
            if let Some(barrier) = barriers.get(event.colliding_entity) {
                let player_motion = motion_2ds.get_mut(event.player_entity).unwrap();
                let player_health = healths.get_mut(event.player_entity).unwrap();

                barrier_collision(player_motion, barrier);

                player_health.value -= barrier.damage;

                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["force_field"].clone(),
                });
            }
        }
    }
}
