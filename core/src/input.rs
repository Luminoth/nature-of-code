use processing::MouseButton;
use processing::Screen;

static mut MOUSE_LEFT_IS_PRESSED: bool = false;
static mut MOUSE_RIGHT_IS_PRESSED: bool = false;
static mut MOUSE_CENTER_IS_PRESSED: bool = false;

pub(crate) fn update(screen: &mut Screen) {
    if screen.mouse_press(MouseButton::Left) {
        unsafe { MOUSE_LEFT_IS_PRESSED = true };
    } else if screen.mouse_release(MouseButton::Left) {
        unsafe { MOUSE_LEFT_IS_PRESSED = false };
    }

    if screen.mouse_press(MouseButton::Right) {
        unsafe { MOUSE_RIGHT_IS_PRESSED = true };
    } else if screen.mouse_release(MouseButton::Right) {
        unsafe { MOUSE_RIGHT_IS_PRESSED = false };
    }

    if screen.mouse_press(MouseButton::Middle) {
        unsafe { MOUSE_CENTER_IS_PRESSED = true };
    } else if screen.mouse_release(MouseButton::Middle) {
        unsafe { MOUSE_CENTER_IS_PRESSED = false };
    }
}

pub fn mouse_is_pressed() -> bool {
    unsafe { MOUSE_LEFT_IS_PRESSED || MOUSE_RIGHT_IS_PRESSED || MOUSE_CENTER_IS_PRESSED }
}
