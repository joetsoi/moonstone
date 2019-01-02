use specs::{Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage};

use crate::animation::SpriteData;
use crate::combat::components::{Action, Draw, Position, State, Velocity};

pub struct StateUpdater;

impl<'a> System<'a> for StateUpdater {
    type SystemData = (
        WriteStorage<'a, State>,
        ReadStorage<'a, Position>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, SpriteData>,
    );

    fn run(&mut self, (mut state, position, entities, updater, sprite_data): Self::SystemData) {
        use specs::Join;
        let sprites = &sprite_data.sprites;

        for (state, entity) in (&mut state, &*entities).join() {
            // Handle dagger throwing.
            if state.action.is_throw_dagger() && state.ticks == 4 {
                let position: Option<&Position> = position.get(entity);
                if let Some(position) = position {
                    let dagger = entities.create();
                    updater.insert(
                        dagger,
                        Position {
                            x: position.x + 5 * state.direction as i32,
                            y: position.y,
                        },
                    );
                    updater.insert(
                        dagger,
                        Velocity {
                            x: 10 * state.direction as i32, //TODO: check velocity.
                            y: 0,
                        },
                    );
                    let frame = sprites
                        .get("dagger")
                        .and_then(|s| s.animations.get("fly"))
                        .and_then(|a| a.frames.get(0))
                        .expect("dagger entity yaml not loaded");
                    updater.insert(
                        dagger,
                        Draw {
                            frame: frame.clone(),
                            animation: "fly".to_string(),
                            resource_name: "dagger".to_string(),
                            direction: state.direction,
                        },
                    );
                }
            }

            match state.action {
                Action::Attack(..) | Action::Hit(..) | Action::AttackRecovery => {
                    state.ticks += 1;
                    if state.ticks == state.length && state.length != 0 {
                        // assuming: that all states that match this arm have a
                        // length > 1 means we don't need to update state.length
                        // in the ResolveCollisions with the animation length
                        // whenever that system changes state of an entity.
                        if state.ticks > 1 {
                            state.action = Action::Idle;
                        }
                    }
                }
                Action::Defend(..) => {
                    // defend actions are 1 frame long, so we switch to idle
                    // immediately
                    state.action = Action::Idle;
                }
                Action::Death => {
                    state.ticks += 1;
                    if state.ticks == state.length && state.length != 0 {
                        if state.ticks > 1 {
                            state.action = Action::Dead;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
