use processing::errors::ProcessingErr;
use processing::Screen;
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;

type World = b2::World<NoUserData>;

fn setup<'a>() -> Result<(Screen<'a>, World), ProcessingErr> {
    let screen = core::create_canvas(640, 360)?;

    let gravity = b2::Vec2 { x: 0., y: -9.81 };
    let world = World::new(&gravity);

    Ok((screen, world))
}

fn draw(screen: &mut Screen, _world: &World, _: f64) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    core::b2d_run(setup, draw)?;

    Ok(())
}
