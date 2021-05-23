use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;

type World = b2::World<NoUserData>;

#[derive(Debug)]
struct Boundary {
    body: Option<b2::BodyHandle>,

    w: f64,
    h: f64,
}

impl Boundary {
    fn new(world: &mut World, screen: &Screen, x: f64, y: f64, w: f64, h: f64) -> Self {
        let mut bd = b2::BodyDef::new();
        bd.body_type = b2::BodyType::Static;
        bd.position = core::coord_pixels_to_world(&screen, x, y);
        let body = world.create_body(&bd);

        let ps = b2::PolygonShape::new_box(
            core::scalar_pixels_to_world(w / 2.0) as f32,
            core::scalar_pixels_to_world(h / 2.0) as f32,
        );

        world.body_mut(body).create_fast_fixture(&ps, 1.0);

        Self {
            body: Some(body),
            w,
            h,
        }
    }

    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        if let Some(body) = self.body {
            let body = world.body(body);
            let pos = core::get_body_pixel_coord(screen, &body);

            core::fill_grayscale(screen, 0.0);
            core::stroke_grayscale(screen, 0.0);

            screen.rect_mode(&core::shapes::RectMode::Center.to_string());
            core::shapes::rect(screen, pos.x as f64, pos.y as f64, self.w, self.h)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct BoxBox {
    body: Option<b2::BodyHandle>,

    w: f64,
    h: f64,
}

impl BoxBox {
    fn new(world: &mut World, screen: &Screen, x: f64, y: f64, w: f64, h: f64) -> Self {
        let mut bd = b2::BodyDef::new();
        bd.body_type = b2::BodyType::Dynamic;
        bd.position = core::coord_pixels_to_world(&screen, x, y);
        let body = world.create_body(&bd);

        let ps = b2::PolygonShape::new_box(
            core::scalar_pixels_to_world(w / 2.0) as f32,
            core::scalar_pixels_to_world(h / 2.0) as f32,
        );

        let mut fd = b2::FixtureDef::new();
        fd.density = 1.0;
        fd.friction = 0.3;
        fd.restitution = 0.5;

        world.body_mut(body).create_fixture(&ps, &mut fd);

        Self {
            body: Some(body),
            w,
            h,
        }
    }

    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        if let Some(body) = self.body {
            let body = world.body(body);
            let pos = core::get_body_pixel_coord(screen, &body);
            let a = body.angle();

            core::fill_grayscale(screen, 175.0);
            core::stroke_grayscale(screen, 0.0);

            screen.push_matrix();

            core::translate(screen, pos.x as f64, pos.y as f64);
            core::rotate(screen, -a as f64);

            screen.rect_mode(&core::shapes::RectMode::Center.to_string());
            core::shapes::rect(screen, 0.0, 0.0, self.w, self.h)?;

            screen.pop_matrix();
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn kill(&mut self, world: &mut World) {
        let body = self.body.take();
        if let Some(body) = body {
            world.destroy_body(body);
        }
    }
}

fn setup<'a>() -> Result<(Screen<'a>, World), ProcessingErr> {
    let screen = core::create_canvas(400, 300)?;

    let gravity = b2::Vec2 { x: 0., y: -9.81 };
    let world = World::new(&gravity);

    Ok((screen, world))
}

fn draw(
    screen: &mut Screen,
    world: &mut World,
    _: f64,
    boxes: &mut Vec<BoxBox>,
    boundaries: &mut Vec<Boundary>,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    if core::input::mouse_is_pressed() {
        let x = screen.mouse_x();
        let y = screen.mouse_y();
        boxes.push(BoxBox::new(world, screen, x, y, 16.0, 16.0));
    }

    for boxbox in boxes.iter() {
        boxbox.display(screen, world)?;
    }

    for boundary in boundaries.iter() {
        boundary.display(screen, world)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let boxes = Rc::new(RefCell::new(None));
    let boundaries = Rc::new(RefCell::new(None));

    core::b2d_run(
        || {
            let (screen, mut world) = setup()?;

            *boxes.borrow_mut() = Some(vec![]);

            *boundaries.borrow_mut() = Some(vec![Boundary::new(
                &mut world,
                &screen,
                screen.width() as f64 / 2.0 - 50.0,
                screen.height() as f64 / 2.0,
                100.0,
                10.0,
            )]);

            Ok((screen, world))
        },
        |screen, world, dt| {
            draw(
                screen,
                world,
                dt,
                boxes.borrow_mut().as_mut().unwrap(),
                boundaries.borrow_mut().as_mut().unwrap(),
            )
        },
    )?;

    Ok(())
}
