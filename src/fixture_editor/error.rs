use conrod::{Canvas, Text, Frameable, Colorable, Sizeable, Positionable, Widget, Button, Labelable};
use piston_window::UpdateEvent;

use structures::ui::colors::FlatColor;
use structures::ui::window::{create_window};

widget_ids! {
    CANVAS,
    TEXT,
    BUTTON,
}

pub struct ErrorWindow {

}

impl ErrorWindow {
    pub fn start(title: String, text: String) {
        let (mut window, mut conrod_ui) = match create_window(title, (400, 100), 30, true) {
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
                    .color(FlatColor::clouds())
                    .set(CANVAS, &mut conrod_ui);

                Text::new(&text)
                    .middle_of(CANVAS)
                    .font_size(15)
                    .color(FlatColor::gray())
                    .set(TEXT, &mut conrod_ui);

                Button::new()
                    .w_h(100.0, 30.0)
                    .label("Ok")
                    .label_font_size(15)
                    .bottom_right_of(CANVAS)
                    .react(|| {
                        println!("How can I close this window?");
                    }) //TODO
                    .set(BUTTON, &mut conrod_ui);
            }));
        };
    }
}
