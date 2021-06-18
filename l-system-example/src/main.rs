use std::cell::RefCell;
use std::rc::Rc;

use processing::errors::ProcessingErr;
use processing::Screen;

#[derive(Debug)]
struct Rule {
    a: char,
    b: String,
}

impl Rule {
    fn new(a: char, b: impl Into<String>) -> Self {
        Self { a, b: b.into() }
    }
}

#[derive(Debug)]
struct Turtle {
    todo: String,
    len: f64,
    theta: f64,
}

impl Turtle {
    fn new(s: impl Into<String>, len: f64, theta: f64) -> Self {
        Self {
            todo: s.into(),
            len,
            theta,
        }
    }

    fn display(&self, screen: &mut Screen) -> Result<(), ProcessingErr> {
        core::stroke_grayscale_alpha(screen, 0.0, 175.0);

        for c in self.todo.chars() {
            match c {
                'F' | 'f' => {
                    core::shapes::line(screen, 0.0, 0.0, self.len, 0.0)?;
                    core::translate(screen, self.len, 0.0);
                }
                'G' | 'g' => {
                    core::translate(screen, self.len, 0.0);
                }
                '+' => {
                    core::rotate(screen, self.theta);
                }
                '-' => {
                    core::rotate(screen, -self.theta);
                }
                '[' => {
                    screen.push_matrix();
                }
                ']' => {
                    screen.pop_matrix();
                }
                _ => {
                    panic!("Invalid alphabet {}", c);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct LSystem {
    sentence: String,
    ruleset: Vec<Rule>,
    generation: usize,
}

impl LSystem {
    fn new(axiom: impl Into<String>, r: impl Into<Vec<Rule>>) -> Self {
        Self {
            sentence: axiom.into(),
            ruleset: r.into(),
            generation: 0,
        }
    }

    fn generate(&mut self) {
        let mut nextgen = String::new();
        for c in self.sentence.chars() {
            let mut replace = c.into();
            for rule in self.ruleset.iter() {
                if rule.a == c {
                    replace = rule.b.clone();
                    break;
                }
            }
            nextgen += replace.as_str();
        }
        self.sentence = nextgen;
        self.generation += 1;
    }
}

fn setup<'a>() -> Result<Screen<'a>, ProcessingErr> {
    core::create_canvas(600, 600)
}

fn draw(
    screen: &mut Screen,
    _: f64,
    lsys: &mut LSystem,
    turtle: &mut Turtle,
) -> Result<(), ProcessingErr> {
    core::background_grayscale(screen, 255.0);

    if core::input::mouse_is_pressed() {
        lsys.generate();

        turtle.todo = lsys.sentence.clone();
        turtle.len *= 0.5;
    }

    core::translate(screen, screen.width() as f64 / 2.0, screen.height() as f64);
    turtle.display(screen)?;

    Ok(())
}

fn main() -> Result<(), ProcessingErr> {
    let lsys = Rc::new(RefCell::new(None));
    let turtle = Rc::new(RefCell::new(None));

    core::run(
        || {
            let screen = setup()?;

            let ruleset = vec![Rule::new('F', "FF+[+F-F-F]-[-F+F+F]")];

            *lsys.borrow_mut() = Some(LSystem::new("F", ruleset));

            *turtle.borrow_mut() = Some(Turtle::new(
                lsys.borrow().as_ref().unwrap().sentence.clone(),
                screen.width() as f64 / 4.0,
                25.0f64.to_radians(),
            ));

            Ok(screen)
        },
        |screen, dt| {
            draw(
                screen,
                dt,
                lsys.borrow_mut().as_mut().unwrap(),
                turtle.borrow_mut().as_mut().unwrap(),
            )
        },
    )?;

    Ok(())
}
