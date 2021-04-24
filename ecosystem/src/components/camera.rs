//! Camera components
//!
//! bevy doesn't provide a non-pixel-sized orthographic camera
//! so this one is based on the [cookbook example](https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#custom-camera-projection)
//! updated to use an "orthographic size" like Unity's orthographic camera

use bevy::prelude::*;
use bevy::render::camera::{
    Camera, CameraProjection, DepthCalculation, VisibleEntities, WindowOrigin,
};
use derivative::Derivative;

/// Orthographic camera projection
#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct OrthoProjection {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,

    near: f32,

    #[derivative(Default(value = "1000.0"))]
    far: f32,

    // half-height
    #[derivative(Default(value = "1.0"))]
    size: f32,

    #[derivative(Default(value = "WindowOrigin::Center"))]
    pub window_origin: WindowOrigin,
}

impl CameraProjection for OrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let aspect = width / height;
        info!("aspect ratio {}", aspect);

        match self.window_origin {
            WindowOrigin::Center => {
                self.left = -aspect * self.size;
                self.right = aspect * self.size;
                self.bottom = -self.size;
                self.top = self.size;
            }
            WindowOrigin::BottomLeft => {
                self.left = 0.0;
                self.right = aspect * self.size * 2.0;
                self.bottom = 0.0;
                self.top = self.size * 2.0;
            }
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

/// Orthographic camera component bundle
#[derive(Bundle)]
pub struct CameraOrtho2dBundle {
    pub camera: Camera,
    pub orthographic_projection: OrthoProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl CameraOrtho2dBundle {
    pub fn new(size: f32) -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;
        Self {
            camera: Camera {
                // have to use one of the internal magic constants
                // since bevy relies on them internally for rendering
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_owned()),
                ..Default::default()
            },
            orthographic_projection: OrthoProjection {
                size,
                far,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, far - 0.1)),
            global_transform: Default::default(),
        }
    }
}

impl Default for CameraOrtho2dBundle {
    fn default() -> Self {
        Self::new(1.0)
    }
}
