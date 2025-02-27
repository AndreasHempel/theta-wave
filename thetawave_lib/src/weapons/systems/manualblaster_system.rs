use crate::weapons::components::ManualFireComponent;
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

/// Handles firing of weapons using input
pub struct ManualBlasterSystem;

impl<'s> System<'s> for ManualBlasterSystem {
    /// Data used by the system
    type SystemData = (Read<'s, Time>, WriteStorage<'s, ManualFireComponent>);

    /// System game logic
    fn run(&mut self, (time, mut manual_fires): Self::SystemData) {
        for manual_fire in (&mut manual_fires).join() {
            if manual_fire.timer > 0.0 && !manual_fire.ready {
                manual_fire.timer -= time.delta_seconds();
            } else if !manual_fire.ready {
                manual_fire.timer = manual_fire.period;
                manual_fire.ready = true;
            }
        }
    }
}
