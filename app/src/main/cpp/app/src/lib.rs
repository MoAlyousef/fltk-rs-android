use fltk::*;
use fltk::{prelude::*};
use fltk::enums::{Color, FrameType};

#[no_mangle]
pub extern "C" fn main() {
    let app = app::App::default();
    let mut win = window::AndroidWindow::default();
    let mut frm = frame::Frame::new(220, 300, 160, 80, "0");
    let mut but = button::Button::new(220, 600, 160, 80, "Click Me");
    win.end();
    win.show();

    // Theming
    frm.set_label_size(50);
    but.set_color(Color::from_u32(0x8E24AA));
    but.set_selection_color(Color::from_u32(0x520e63));
    but.set_label_size(36);
    but.set_label_color(Color::White);
    but.set_frame(FrameType::RFlatBox);
    but.clear_visible_focus();

    but.set_callback(move |_| {
        let mut val: i32 = frm.label().parse().unwrap();
        val += 1;
        frm.set_label(&val.to_string());
        app::redraw();
    });

    app.run().unwrap();
}

