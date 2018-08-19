use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;
use specs::{ReadStorage, System, WriteStorage};

use crate::combat::components::{Command, Controller, Intent, MoveCommand};

const IDLE: (i32, i32) = (0, 0);
const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const LEFT_UP: (i32, i32) = (-1, -1);
const RIGHT_UP: (i32, i32) = (1, -1);
const LEFT_DOWN: (i32, i32) = (-1, 1);
const RIGHT_DOWN: (i32, i32) = (1, 1);

lazy_static! {
    static ref controller_to_move: HashMap<(i32, i32), Command> = hashmap!{
        IDLE => Command::Idle,
        UP => Command::Move(MoveCommand::TryMoveUp),
        DOWN => Command::Move(MoveCommand::TryMoveDown),
        LEFT => Command::Move(MoveCommand::TryMoveLeft),
        RIGHT => Command::Move(MoveCommand::TryMoveRight),
        LEFT_UP => Command::Move(MoveCommand::TryMoveLeftUp),
        RIGHT_UP => Command::Move(MoveCommand::TryMoveRightUp),
        LEFT_DOWN => Command::Move(MoveCommand::TryMoveLeftDown),
        RIGHT_DOWN => Command::Move(MoveCommand::TryMoveRightDown),
    };
}

pub struct Commander;

impl<'a> System<'a> for Commander {
    type SystemData = (ReadStorage<'a, Controller>, WriteStorage<'a, Intent>);

    fn run(&mut self, (controller, mut intent): Self::SystemData) {
        use specs::Join;

        for (controller, intent) in (&controller, &mut intent).join() {
            intent.command = controller_to_move[&(controller.x, controller.y)];
            //println!("{:?}", intent);
        }
    }
}
