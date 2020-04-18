use crate::{components::Item, resources::SpriteResource};
use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};

pub fn spawn_item(
    entities: &Entities,
    item_resource: &ReadExpect<SpriteResource>,
    item: Item,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite = SpriteRender {
        sprite_sheet: item_resource.items_sprite_sheet.clone(),
        sprite_number: item.sprite_index,
    };
    super::spawn_sprite_entity(&entities, sprite, item, spawn_position, &lazy_update);
}
