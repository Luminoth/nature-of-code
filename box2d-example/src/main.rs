use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;
use wrapped2d::b2;
use wrapped2d::b2::Joint;
use wrapped2d::dynamics::world::callbacks::ContactAccess;
use wrapped2d::user_data::UserDataTypes;

struct CustomUserData;

impl UserDataTypes for CustomUserData {
    // TODO: no clue what we can actually use here
    type BodyData = ();
    type JointData = ();
    type FixtureData = ();
}

type World = b2::World<CustomUserData>;

#[derive(Debug)]
struct Windmill {
    box1: BoxBox,
    box2: BoxBox,

    joint: b2::JointHandle,
}

impl Windmill {
    #[allow(dead_code)]
    fn new(world: &mut World, screen: &Screen, x: f64, y: f64) -> Self {
        let box1 = BoxBox::new(world, screen, x, y, 120.0, 10.0);
        let box2 = BoxBox::new(world, screen, x, y, 10.0, 40.0);

        let mut rjd = b2::RevoluteJointDef::new(box1.body.unwrap(), box2.body.unwrap());
        rjd.init(
            world,
            box1.body.unwrap(),
            box2.body.unwrap(),
            world.body(box1.body.unwrap()).world_center(),
        );
        rjd.motor_speed = std::f32::consts::PI * 2.0;
        rjd.max_motor_torque = 1000.0;
        rjd.enable_motor = true;
        let joint = world.create_joint(&rjd);

        Self { box1, box2, joint }
    }

    #[allow(dead_code)]
    fn toggle_motor(&self, world: &mut World) {
        let joint = &mut *world.joint_mut(self.joint);
        match joint.deref_mut() {
            b2::UnknownJoint::Revolute(joint) => {
                joint.enable_motor(!joint.is_motor_enabled());
            }
            _ => panic!("unexpected joint type {:?}", joint.get_type()),
        }
    }

    #[allow(dead_code)]
    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        self.box1.display(screen, world)?;
        self.box2.display(screen, world)?;

        Ok(())
    }
}

#[derive(Debug)]
struct Surface {
    body: b2::BodyHandle,
    fixture: b2::FixtureHandle,
}

impl Surface {
    fn new(world: &mut World, screen: &Screen, vertices: impl AsRef<Vec<b2::Vec2>>) -> Self {
        // convert to world coords
        let mut surface = Vec::with_capacity(vertices.as_ref().len());
        for vertex in vertices.as_ref() {
            surface.push(core::coord_pixels_to_world(
                &screen,
                vertex.x as f64,
                vertex.y as f64,
            ));
        }

        let mut bd = b2::BodyDef::new();
        bd.body_type = b2::BodyType::Static;
        let body = world.create_body_with(&bd, ());

        let cs = b2::ChainShape::new_chain(&surface);
        let fixture = world.body_mut(body).create_fast_fixture(&cs, 1.0);

        Self { body, fixture }
    }

    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        let body = world.body(self.body);
        let fixture = body.fixture(self.fixture);
        let shape = fixture.shape();

        screen.fill_off();
        screen.stroke_weight(1.0);
        core::stroke_grayscale(screen, 0.0);

        // TODO: beginShape();

        let vertices = match &*shape {
            b2::UnknownShape::Chain(shape) => shape.vertices(),
            _ => panic!("unexpected shape type {:?}", fixture.shape_type()),
        };

        for vertex in vertices {
            let _v = core::vector_world_to_pixels(screen, *vertex);
            //vertex(v.x, v.y);
        }

        // TODO: endShape(CLOSE);

        Ok(())
    }
}

#[derive(Debug)]
struct Boundary {
    body: b2::BodyHandle,

    w: f64,
    h: f64,
}

impl Boundary {
    fn new(world: &mut World, screen: &Screen, x: f64, y: f64, w: f64, h: f64) -> Self {
        let mut bd = b2::BodyDef::new();
        bd.body_type = b2::BodyType::Static;
        bd.position = core::coord_pixels_to_world(&screen, x, y);
        let body = world.create_body_with(&bd, ());

        let ps = b2::PolygonShape::new_box(
            core::scalar_pixels_to_world(w / 2.0) as f32,
            core::scalar_pixels_to_world(h / 2.0) as f32,
        );

        world.body_mut(body).create_fast_fixture(&ps, 1.0);

        Self { body, w, h }
    }

    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        let body = world.body(self.body);
        let pos = core::get_body_pixel_coord(screen, &body);

        core::fill_grayscale(screen, 0.0);
        core::stroke_grayscale(screen, 0.0);

        screen.rect_mode(&core::shapes::RectMode::Center.to_string());
        core::shapes::rect(screen, pos.x as f64, pos.y as f64, self.w, self.h)?;

        Ok(())
    }
}

#[derive(Debug)]
struct Pair {
    p1: BoxBox,
    p2: BoxBox,

    len: f64,
}

impl Pair {
    fn new(world: &mut World, screen: &Screen, x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();

        let len = 32.0;

        let p1 = BoxBox::new(world, screen, x, y, 16.0, 16.0);
        let p2 = BoxBox::new(
            world,
            screen,
            x + rng.gen_range(-1.0..=1.0) as f64,
            y + rng.gen_range(-1.0..=1.0) as f64,
            16.0,
            16.0,
        );

        let mut djd = b2::DistanceJointDef::new(p1.body.unwrap(), p2.body.unwrap());
        djd.length = core::scalar_pixels_to_world(len) as f32;
        world.create_joint(&djd);

        Self { p1, p2, len }
    }

    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        let p1 = world.body(self.p1.body.unwrap());
        let pos1 = core::get_body_pixel_coord(screen, &p1);

        let p2 = world.body(self.p2.body.unwrap());
        let pos2 = core::get_body_pixel_coord(screen, &p2);

        core::stroke_grayscale(screen, 0.0);

        core::shapes::line(
            screen,
            pos1.x as f64,
            pos1.y as f64,
            pos2.x as f64,
            pos2.y as f64,
        )?;

        self.p1.display(screen, world)?;
        self.p2.display(screen, world)?;

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
        let body = world.create_body_with(&bd, ());

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

    #[allow(dead_code)]
    fn apply_force(&self, world: World, force: &b2::Vec2) {
        let mut body = world.body_mut(self.body.unwrap());

        let pos = *body.world_center();
        body.apply_force(force, &pos, true);
    }

    fn display(&self, screen: &mut Screen, world: &World) -> Result<(), ProcessingErr> {
        let body = world.body(self.body.unwrap());
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

        Ok(())
    }

    #[allow(dead_code)]
    fn kill(&mut self, world: &mut World) {
        world.destroy_body(self.body.take().unwrap());
    }
}

struct ContactListener;

impl b2::ContactListener<CustomUserData> for ContactListener {
    fn begin_contact(&mut self, _cp: ContactAccess<CustomUserData>) {
        // TODO:
    }
}

fn setup<'a>() -> Result<(Screen<'a>, World), ProcessingErr> {
    let screen = core::create_canvas(400, 300)?;

    let gravity = b2::Vec2 { x: 0., y: -9.81 };
    let mut world = World::new(&gravity);

    world.set_contact_listener(Box::new(ContactListener));

    Ok((screen, world))
}

fn draw(
    screen: &mut Screen,
    world: &mut World,
    _: f64,
    boxes: &mut Vec<BoxBox>,
    pairs: impl AsRef<[Pair]>,
    boundaries: impl AsRef<[Boundary]>,
    surfaces: impl AsRef<[Surface]>,
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

    for pair in pairs.as_ref().iter() {
        pair.display(screen, world)?;
    }

    for boundary in boundaries.as_ref().iter() {
        boundary.display(screen, world)?;
    }

    for surface in surfaces.as_ref().iter() {
        surface.display(screen, world)?;
    }

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let boxes = Rc::new(RefCell::new(None));
    let pairs = Rc::new(RefCell::new(None));
    let boundaries = Rc::new(RefCell::new(None));
    let surfaces = Rc::new(RefCell::new(None));

    core::b2d_run(
        || {
            let (screen, mut world) = setup()?;

            *boxes.borrow_mut() = Some(vec![]);

            let hw = screen.width() as f64 / 2.0;
            let hh = screen.height() as f64 / 2.0;

            *pairs.borrow_mut() = Some(vec![Pair::new(&mut world, &screen, hw + 30.0, hh - 30.0)]);

            *boundaries.borrow_mut() = Some(vec![Boundary::new(
                &mut world,
                &screen,
                hw - 50.0,
                hh - 50.0,
                100.0,
                10.0,
            )]);

            let hw = screen.width() as f32 / 2.0;
            let hh = screen.height() as f32 / 2.0;

            let vertices = vec![
                b2::Vec2 {
                    x: 0.0,
                    y: hh + 50.0,
                },
                b2::Vec2 {
                    x: hw,
                    y: hh + 50.0,
                },
                b2::Vec2 { x: hw, y: hh },
            ];

            *surfaces.borrow_mut() = Some(vec![Surface::new(&mut world, &screen, vertices)]);

            Ok((screen, world))
        },
        |screen, world, dt| {
            draw(
                screen,
                world,
                dt,
                boxes.borrow_mut().as_mut().unwrap(),
                pairs.borrow_mut().as_mut().unwrap(),
                boundaries.borrow_mut().as_mut().unwrap(),
                surfaces.borrow_mut().as_mut().unwrap(),
            )
        },
    )?;

    Ok(())
}
