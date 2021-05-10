//! Physics components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use num_traits::Float;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

#[derive(Debug, Default, Copy, Clone)]
struct Derivative {
    acceleration: Vec3,
    velocity: Vec3,
}

impl Derivative {
    fn evaluate(acceleration: Vec3, velocity: Vec3, dt: f32, derivative: Self) -> Self {
        Self {
            velocity: velocity + derivative.acceleration * dt,
            acceleration,
        }
    }
}

/// RK4 integration: https://gafferongames.com/post/integration_basics/
#[allow(dead_code)]
pub fn rk4_integrate(transform: &mut Transform, acceleration: Vec3, velocity: &mut Vec3, dt: f32) {
    // sample derivative at four points
    let a = Derivative::evaluate(acceleration, *velocity, 0.0, Derivative::default());
    let b = Derivative::evaluate(acceleration, *velocity, dt * 0.5, a);
    let c = Derivative::evaluate(acceleration, *velocity, dt * 0.5, b);
    let d = Derivative::evaluate(acceleration, *velocity, dt, c);

    // taylor expansion weighted sum
    let dvdt =
        1.0 / 6.0 * (a.acceleration + 2.0 * (b.acceleration + c.acceleration) + d.acceleration);
    let dxdt = 1.0 / 6.0 * (a.velocity + 2.0 * (b.velocity + c.velocity) + d.velocity);

    *velocity += dvdt * dt;
    transform.translation += dxdt * dt;
}

/// Explicit Euler integration
#[allow(dead_code)]
pub fn explicit_euler_integrate(
    transform: &mut Transform,
    acceleration: Vec3,
    velocity: &mut Vec3,
    dt: f32,
) {
    transform.translation += *velocity * dt;
    *velocity += acceleration * dt;
}

/// Semi-implicit Euler integration
#[allow(dead_code)]
pub fn sympletic_euler_integrate(
    transform: &mut Transform,
    acceleration: Vec3,
    velocity: &mut Vec3,
    dt: f32,
) {
    *velocity += acceleration * dt;
    transform.translation += *velocity * dt;
}

/// Rigidbody state
#[derive(Debug, Inspectable)]
pub struct Rigidbody {
    #[inspectable(ignore)]
    pub(crate) previous_position: Vec3,

    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub drag: f32,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            previous_position: Vec3::default(),

            acceleration: Vec3::default(),
            velocity: Vec3::default(),
            mass: 1.0,
            drag: 0.0,
        }
    }
}

impl Rigidbody {
    /// Gets the rigidbody speed
    #[allow(dead_code)]
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    /// Gets the rigidbody speed squared
    #[allow(dead_code)]
    pub fn speed_squared(&self) -> f32 {
        self.velocity.length_squared()
    }

    /// Contain a rigidbody inside bounds
    #[allow(dead_code)]
    pub fn contain(&mut self, transform: &mut Transform, min: Vec2, max: Vec2, min_distance: f32) {
        // unwind to our previous position, if we can
        // otherwise clamp to the min / max minus a little fudge

        if transform.translation.x <= min.x {
            transform.translation.x = if self.previous_position.x <= min.x {
                min.x + min_distance
            } else {
                self.previous_position.x
            };
            self.velocity.x = 0.0;
        } else if transform.translation.x >= max.x {
            transform.translation.x = if self.previous_position.x >= max.x {
                max.x - min_distance
            } else {
                self.previous_position.x
            };
            self.velocity.x = 0.0;
        }

        if transform.translation.y <= min.y {
            transform.translation.y = if self.previous_position.y <= min.y {
                min.y + min_distance
            } else {
                self.previous_position.y
            };
            self.velocity.y = 0.0;
        } else if transform.translation.y >= max.y {
            transform.translation.y = if self.previous_position.y >= max.y {
                max.y - min_distance
            } else {
                self.previous_position.y
            };
            self.velocity.y = 0.0;
        }
    }

    fn attract_repel_force(&self, ab: Vec2, acceleration: f32, min_distance: f32) -> Vec2 {
        let distance = Float::max(min_distance, ab.length());
        let direction = ab.normalize_or_zero();
        let magnitude = (acceleration * self.mass) / (distance * distance);

        direction * magnitude
    }

    /// Repel a rigidbody inside bounds
    #[allow(dead_code)]
    pub fn bounds_repel(
        &mut self,
        transform: &Transform,
        min: Vec2,
        max: Vec2,
        acceleration: f32,
        min_distance: f32,
    ) {
        let force = self.attract_repel_force(
            Vec2::new(transform.translation.x - min.x, 0.0),
            acceleration,
            min_distance,
        );
        self.apply_force(force);

        let force = self.attract_repel_force(
            Vec2::new(transform.translation.x - max.x, 0.0),
            acceleration,
            min_distance,
        );
        self.apply_force(force);

        let force = self.attract_repel_force(
            Vec2::new(0.0, transform.translation.y - min.y),
            acceleration,
            min_distance,
        );
        self.apply_force(force);

        let force = self.attract_repel_force(
            Vec2::new(0.0, transform.translation.y - max.y),
            acceleration,
            min_distance,
        );
        self.apply_force(force);
    }

    /// Repel a a rigidbody away from a point
    #[allow(dead_code)]
    pub fn repel(
        &mut self,
        transform: &Transform,
        point: Vec2,
        acceleration: f32,
        min_distance: f32,
    ) {
        let force = self.attract_repel_force(
            transform.translation.truncate() - point,
            acceleration,
            min_distance,
        );
        self.apply_force(force);
    }

    /// Attact a a rigidbody toward a point
    #[allow(dead_code)]
    pub fn attract(
        &mut self,
        transform: &Transform,
        point: Vec2,
        acceleration: f32,
        min_distance: f32,
    ) {
        let force = self.attract_repel_force(
            point - transform.translation.truncate(),
            acceleration,
            min_distance,
        );
        self.apply_force(force);
    }

    /// Applies a force to the rigidbody
    pub fn apply_force(&mut self, force: Vec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        self.acceleration += force.extend(0.0);
    }

    /// Update a rigidbody
    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        // save our current position in case we need to unwind
        self.previous_position = transform.translation;

        //sympletic_euler_integrate(transform, self.acceleration, &mut self.velocity, dt);
        rk4_integrate(transform, self.acceleration, &mut self.velocity, dt);

        self.acceleration = Vec3::default();
    }
}

/* Colliders */

/// Colliders on the same layer collide
#[derive(Debug, Copy, Clone, Eq, PartialEq, Inspectable)]
pub enum CollisionLayer {
    Ground,
    Water,
    Air,
}

impl Default for CollisionLayer {
    fn default() -> Self {
        CollisionLayer::Ground
    }
}

/// AABB collider
#[derive(Debug, Copy, Clone, Inspectable)]
pub struct BoxCollider {
    pub center: Vec2,
    pub size: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            center: Vec2::default(),
            size: Vec2::new(1.0, 1.0),
        }
    }
}

impl BoxCollider {
    /// Creates a new BoxCollider at the given center with the given size
    pub fn new(center: Vec2, size: Vec2) -> Self {
        Self { center, size }
    }

    /// Returns true if the BoxCollider intersects the other BoxCollider
    pub fn intersects_box(&self, transform: &Transform, other: (&Transform, &BoxCollider)) -> bool {
        let position = transform.translation.truncate();
        let min = position - self.size;
        let max = position + self.size;

        let oposition = other.0.translation.truncate();
        let omin = oposition - other.1.size;
        let omax = oposition - other.1.size;

        (min.x <= omax.x && max.x >= omin.x) && (min.y <= omax.y && max.y >= omin.y)
    }
}

/// Colliders collide
#[derive(Debug, Copy, Clone, Inspectable)]
pub enum Collider {
    Box(BoxCollider, CollisionLayer),
}

impl Default for Collider {
    fn default() -> Self {
        Collider::Box(BoxCollider::default(), CollisionLayer::default())
    }
}

impl Collider {
    /// Returns the size of the collider
    pub fn size(&self) -> Vec2 {
        match self {
            Collider::Box(collider, _) => collider.size,
        }
    }

    fn collides_layers(&self, layer: CollisionLayer) -> bool {
        match self {
            Collider::Box(_, slayer) => *slayer == layer,
        }
    }

    fn intersects_box(&self, transform: &Transform, other: (&Transform, &BoxCollider)) -> bool {
        match self {
            Collider::Box(collider, _) => collider.intersects_box(transform, other),
        }
    }

    /// Check if a collider is colliding with another collider
    pub fn collides(&self, transform: &Transform, other: (&Transform, &Collider)) -> bool {
        match self {
            Collider::Box(collider, layer) => {
                other.1.collides_layers(*layer)
                    && other.1.intersects_box(other.0, (transform, collider))
            }
        }
    }

    /// Adjusts the bounds that should contain this collider
    pub fn adjust_container_bounds(&self, min: Vec2, max: Vec2, offset: f32) -> (Vec2, Vec2) {
        let size = self.size();
        let offset = Vec2::splat(offset);
        let min = min + size + offset;
        let max = max - size - offset;

        (min, max)
    }
}

/// Surface state
#[derive(Debug, Inspectable, Default)]
pub struct Surface {
    pub c: f32,
}

impl Surface {
    pub fn update(&self, rigidbody: &mut Rigidbody) {
        let magnitude = self.c;
        let direction = -rigidbody.velocity.normalize_or_zero();

        let friction = direction * magnitude;

        rigidbody.apply_force(friction.truncate());
    }
}

/// Fluid state
#[derive(Debug, Inspectable, Default)]
pub struct Fluid {
    pub density: f32,
}

impl Fluid {
    pub fn update(&self, rigidbody: &mut Rigidbody) {
        let speed_squared = rigidbody.speed_squared();
        let magnitude = 0.5 * self.density * speed_squared * rigidbody.drag;
        let direction = -rigidbody.velocity.normalize_or_zero();

        let drag = direction * magnitude;

        rigidbody.apply_force(drag.truncate());
    }
}

/// Oscillator
#[derive(Debug, Inspectable)]
pub struct Oscillator {
    pub angle: Vec2,
    pub velocity: Vec2,
    pub amplitude: Vec2,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            angle: Vec2::default(),
            velocity: Vec2::default(),
            amplitude: Vec2::splat(1.0),
        }
    }
}

impl Oscillator {
    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        self.angle += self.velocity * dt;

        transform.translation.x = self.angle.x.sin() * self.amplitude.x;
        transform.translation.y = self.angle.y.sin() * self.amplitude.y;
    }
}
