use conrod::{Canvas, Text, Frameable, Colorable, Sizeable, Positionable, Widget, Button, Labelable};
use piston_window::UpdateEvent;

use super::colors::FlatColor;
use super::window::create_window;

widget_ids! {
    CANVAS,
    TEXT,
    BUTTON,
}

trait Unwrap2<T> {
    fn unwrap2(self, msg: &'static str) -> T;
}

impl<T> Unwrap2<T> for Option<T> {
    fn unwrap2(self, msg: &'static str) -> T{
        match self {
            Some(x) => x,
            None => {
                error_message(msg);
                panic!(msg);
            }
        }
    }
}

pub fn error_message(msg: &'static str) { //TODO add send report button
    let (mut window, mut conrod_ui) = match create_window("Error".to_string(), (200, 100), 30, true) {
        Ok(res) => res,
        Err(e) => {
            exit!(3, e);
        }
    };

    while let Some(event) = window.next() {
        conrod_ui.handle_event(&event);
        window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

        event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {
            Canvas::new()
                .frame(1.0)
                .pad(5.0)
                .color(FlatColor::midnight_blue())
                .set(CANVAS, &mut conrod_ui);

            Text::new(msg)
                .middle_of(CANVAS)
                .w(150.0)
                .font_size(15)
                .color(FlatColor::clouds())
                .set(TEXT, &mut conrod_ui);

            Button::new()
                .w_h(100.0, 30.0)
                .label("Quit")
                .label_font_size(15)
                .mid_bottom_of(CANVAS)
                .react(|| {
                    exit!(4, msg.clone());
                })
                .set(BUTTON, &mut conrod_ui);
        }));
    };
}
