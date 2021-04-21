use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use processing::MouseButton;
use processing::Screen;

static MOUSE_LEFT_IS_PRESSED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static MOUSE_RIGHT_IS_PRESSED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static MOUSE_CENTER_IS_PRESSED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub(crate) fn update(screen: &mut Screen) {
    if screen.mouse_press(MouseButton::Left) {
        MOUSE_LEFT_IS_PRESSED.store(true, Ordering::Relaxed);
    } else if screen.mouse_release(MouseButton::Left) {
        MOUSE_LEFT_IS_PRESSED.store(false, Ordering::Relaxed);
    }

    if screen.mouse_press(MouseButton::Right) {
        MOUSE_RIGHT_IS_PRESSED.store(true, Ordering::Relaxed);
    } else if screen.mouse_release(MouseButton::Right) {
        MOUSE_RIGHT_IS_PRESSED.store(false, Ordering::Relaxed);
    }

    if screen.mouse_press(MouseButton::Middle) {
        MOUSE_CENTER_IS_PRESSED.store(true, Ordering::Relaxed);
    } else if screen.mouse_release(MouseButton::Middle) {
        MOUSE_CENTER_IS_PRESSED.store(false, Ordering::Relaxed);
    }
}

pub fn mouse_is_pressed() -> bool {
    MOUSE_LEFT_IS_PRESSED.load(Ordering::Relaxed)
        || MOUSE_RIGHT_IS_PRESSED.load(Ordering::Relaxed)
        || MOUSE_CENTER_IS_PRESSED.load(Ordering::Relaxed)
}
