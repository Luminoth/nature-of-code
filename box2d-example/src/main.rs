use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;

type World = b2::World<NoUserData>;

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64, _world: &World) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let world = Rc::new(RefCell::new(None));

    core::run(
        || {
            let gravity = b2::Vec2 { x: 0., y: -9.81 };
            *world.borrow_mut() = Some(World::new(&gravity));

            setup()
        },
        |screen, dt| draw(screen, dt, world.borrow().as_ref().unwrap()),
    )?;

    Ok(())
}
