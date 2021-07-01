use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

// lower learning constant produces a slower,
// more visually interesting solution
// (default is 0.01)
const C: f32 = 0.00001;

#[derive(Debug)]
struct Neuron {
    location: DVec2,

    connections: Vec<(Rc<RefCell<Neuron>>, f32)>,
}

impl Neuron {
    fn new(x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            connections: vec![],
        }
    }

    fn add_connection(&mut self, b: Rc<RefCell<Neuron>>, weight: f32) {
        self.connections.push((b, weight));
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        // connections
        core::stroke_grayscale(screen, 0.0);
        for b in self.connections.iter() {
            screen.stroke_weight(b.1 * 4.0);

            core::shapes::line(
                screen,
                self.location.x,
                self.location.y,
                b.0.borrow().location.x,
                b.0.borrow().location.y,
            )?;
        }

        // neuron
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 0.0);
        core::shapes::ellipse(screen, self.location.x, self.location.y, 16.0, 16.0)?;

        Ok(())
    }
}

#[derive(Debug)]
struct Network {
    neurons: Vec<Rc<RefCell<Neuron>>>,

    location: DVec2,
}

impl Network {
    fn new(x: f64, y: f64) -> Self {
        Self {
            neurons: vec![],
            location: DVec2::new(x, y),
        }
    }

    fn add_neuron(&mut self, n: Neuron) -> Rc<RefCell<Neuron>> {
        self.neurons.push(Rc::new(RefCell::new(n)));
        self.neurons.last().unwrap().clone()
    }

    fn connect(&self, a: &mut Neuron, b: Rc<RefCell<Neuron>>) {
        let mut rng = rand::thread_rng();

        a.add_connection(b, rng.gen_range(0.0..1.0));
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        screen.push_matrix();

        core::translatev(screen, self.location);

        for neuron in self.neurons.iter() {
            neuron.borrow().display(screen)?;
        }

        screen.pop_matrix();

        Ok(())
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(640, 360)
}

fn draw(screen: &mut Screen, _: f64, network: &Network) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    network.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let network = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let mut n = Network::new(screen.width() as f64 / 2.0, screen.height() as f64 / 2.0);

            let a = n.add_neuron(Neuron::new(-200.0, 0.0));
            let b = n.add_neuron(Neuron::new(0.0, 100.0));
            let c = n.add_neuron(Neuron::new(0.0, -100.0));
            let d = n.add_neuron(Neuron::new(200.0, 0.0));

            n.connect(&mut a.borrow_mut(), b.clone());
            n.connect(&mut a.borrow_mut(), c.clone());
            n.connect(&mut b.borrow_mut(), d.clone());
            n.connect(&mut c.borrow_mut(), d);

            *network.borrow_mut() = Some(n);

            Ok(screen)
        },
        |screen, dt| {
            draw(screen, dt, network.borrow().as_ref().unwrap())?;

            Ok(())
        },
    )?;

    Ok(())
}