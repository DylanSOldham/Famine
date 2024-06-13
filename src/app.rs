extern crate famine;

use famine::Window;

pub fn startup<W: Window>() -> W {
    println!("HELLO FROM THE USER FACING SIDE");
    W::new("ZA APP", 640, 480)
}

pub fn update(window: &impl Window) {
    window.clear(1.0, 1.0, 0.0, 1.0);
}