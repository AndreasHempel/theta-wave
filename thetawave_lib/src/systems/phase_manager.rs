use crate::resources::PhaseManagerResource;
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System, Write},
};

pub struct PhaseManagerSystem;

impl<'s> System<'s> for PhaseManagerSystem {
    type SystemData = (Write<'s, PhaseManagerResource>, Read<'s, Time>);

    fn run(&mut self, (mut phase_manager, time): Self::SystemData) {
        phase_manager.update(time.delta_seconds());
    }
}
