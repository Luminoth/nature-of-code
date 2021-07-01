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

    connections: Vec<Connection>,

    sum: f32,
}

impl Neuron {
    fn new(x: f64, y: f64) -> Self {
        Self {
            location: DVec2::new(x, y),
            connections: vec![],
            sum: 0.0,
        }
    }

    fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    fn feedforward(&mut self, input: f32) {
        self.sum += input;
        if self.sum > 1.0 {
            self.fire();
            self.sum = 0.0;
        }
    }

    fn fire(&self) {
        for connection in self.connections.iter() {
            connection.feedforward(self.sum);
        }
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        core::fill_grayscale(screen, 0.0);
        core::shapes::ellipse(screen, self.location.x, self.location.y, 16.0, 16.0)?;

        for connection in self.connections.iter() {
            connection.display(screen)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Connection {
    a: Rc<RefCell<Neuron>>,
    b: Rc<RefCell<Neuron>>,
    weight: f32,
}

impl Connection {
    fn new(from: Rc<RefCell<Neuron>>, to: Rc<RefCell<Neuron>>, weight: f32) -> Self {
        Self {
            a: from,
            b: to,
            weight,
        }
    }

    fn feedforward(&self, v: f32) {
        self.b.borrow_mut().feedforward(v);
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        screen.stroke_weight(self.weight * 4.0);

        core::shapes::line(
            screen,
            self.a.borrow().location.x,
            self.a.borrow().location.y,
            self.b.borrow().location.x,
            self.b.borrow().location.y,
        )?;

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

    fn connect(&self, a: Rc<RefCell<Neuron>>, b: Rc<RefCell<Neuron>>) {
        let mut rng = rand::thread_rng();

        a.borrow_mut()
            .add_connection(Connection::new(a.clone(), b, rng.gen_range(0.0..1.0)));
    }

    fn feedforward(&self, input: f32) {
        self.neurons[0].borrow_mut().feedforward(input);
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

            n.connect(a.clone(), b.clone());
            n.connect(a, c.clone());
            n.connect(b, d.clone());
            n.connect(c, d);

            let mut rng = rand::thread_rng();
            n.feedforward(rng.gen_range(0.0..1.0));

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
