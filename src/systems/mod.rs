mod spaceship;
mod blast;
mod enemy;
mod enemy_spawn;
mod player_hit;
mod explosion;
mod item;
mod spaceship_movement;
mod enemy_hit;
mod consumable;
mod status_bar;
mod defense;
mod collision_detection;
mod collision_handler;
mod gamemaster;
mod stat_tracker;
mod store;
mod planets;
mod animation;
mod boss;

pub use self::{
    blast::BlastSystem,
    spaceship::SpaceshipSystem,
    enemy::EnemySystem,
    enemy_spawn::SpawnerSystem,
    player_hit::PlayerHitSystem,
    explosion::ExplosionSystem,
    item::ItemSystem,
    spaceship_movement::SpaceshipMovementSystem,
    enemy_hit::EnemyHitSystem,
    consumable::ConsumableSystem,
    status_bar::StatusBarSystem,
    defense::DefenseSystem,
    collision_detection::CollisionDetectionSystem,
    collision_handler::CollisionHandlerSystem,
    gamemaster::GameMasterSystem,
    stat_tracker::StatTrackerSystem,
    store::StoreSystem,
    planets::PlanetsSystem,
    animation::AnimationSystem,
    boss::BossSystem,
};

pub fn hitbox_collide(mut x1: f32, mut y1: f32, mut x2: f32, mut y2: f32, hitbox_width_1: f32, hitbox_height_1: f32, hitbox_width_2: f32, hitbox_height_2: f32) -> bool {
    x1 -= hitbox_width_1 / 2.0;
    y1 -= hitbox_height_1 / 2.0;
    x2 -= hitbox_width_2 / 2.0;
    y2 -= hitbox_height_2 / 2.0 ;

    x1 < (x2 + hitbox_width_2) && (x1 + hitbox_width_1) > x2 && y1 < (y2 + hitbox_height_2) && (y1 + hitbox_height_1) > y2
}
