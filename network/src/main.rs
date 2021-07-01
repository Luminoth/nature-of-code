use std::cell::RefCell;
use std::rc::Rc;

use glam::DVec2;
use processing::errors::ProcessingErr;
use processing::Screen;
use rand::Rng;

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

    fn fire(&mut self) {
        for connection in self.connections.iter_mut() {
            connection.feedforward(self.location, self.sum);
        }
    }

    fn update(&mut self) {
        for connection in self.connections.iter_mut() {
            connection.update();
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

    sending: bool,
    sender: DVec2,
    output: f32,
}

impl Connection {
    fn new(from: Rc<RefCell<Neuron>>, to: Rc<RefCell<Neuron>>, weight: f32) -> Self {
        Self {
            a: from,
            b: to,
            weight,
            sending: false,
            sender: DVec2::default(),
            output: 0.0,
        }
    }

    // location should be == self.a.location
    // we just can't borrow a here
    fn feedforward(&mut self, location: DVec2, v: f32) {
        self.output = v * self.weight;
        self.sender = location;
        self.sending = true;
    }

    fn update(&mut self) {
        if !self.sending {
            return;
        }

        let b = self.b.borrow().location;

        self.sender = self.sender.lerp(b, 0.1);
        let d = self.sender.distance(b);
        if d < 1.0 {
            self.b.borrow_mut().feedforward(self.output);
            self.sending = false;
        }
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale(screen, 0.0);
        screen.stroke_weight(1.0 + self.weight * 4.0);

        let a = &self.a.borrow().location;
        let b = &self.b.borrow().location;
        core::shapes::line(screen, a.x, a.y, b.x, b.y)?;

        if self.sending {
            core::fill_grayscale(screen, 0.0);
            screen.stroke_weight(1.0);
            core::shapes::ellipse(screen, self.sender.x, self.sender.y, 16.0, 16.0)?;
        }

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

    fn update(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.borrow_mut().update();
        }
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

fn draw(screen: &mut Screen, _: f64, network: &mut Network) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    network.update();
    network.display(screen)?;

    if screen.frame_count() % 30 == 0 {
        let mut rng = rand::thread_rng();
        network.feedforward(rng.gen_range(0.0..1.0));
    }

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

            *network.borrow_mut() = Some(n);

            Ok(screen)
        },
        |screen, dt| {
            draw(screen, dt, network.borrow_mut().as_mut().unwrap())?;

            Ok(())
        },
    )?;

    Ok(())
}
