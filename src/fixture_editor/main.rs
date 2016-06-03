#[macro_use] extern crate structures;
#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate piston_window;

use std::thread::{self, JoinHandle};
use conrod::{Canvas, Text, Frameable, Colorable, Sizeable, Positionable, Widget};
use piston_window::UpdateEvent;
use std::any::Any;

use structures::ui::colors::FlatColor;
use structures::ui::window::{create_window, DMXWindow};

widget_ids! {
    CANVAS,
    HELLO_WORLD
}

pub struct FixtureWindow {

}

impl FixtureWindow {
    pub fn start() {
        let (mut window, mut conrod_ui) = match create_window("BitDMX Fixture editor".to_string(), (500, 300), 30, true) {
            Ok(res) => res, Err(e) => {exit!(3, e);}
        };

        while let Some(event) = window.next() {
            conrod_ui.handle_event(&event);
            window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

            event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {
                // Move this into a function and possibly sub-file please...
                Canvas::new()
                    .frame(0.0)
                    .pad(5.0)
                    .color(FlatColor::concrete())
                    .set(CANVAS, &mut conrod_ui);

                Text::new("Hi!")
                    // .w_h(110.0, 110.0)
                    .middle_of(CANVAS)
                    .font_size(100)
                    .color(FlatColor::clouds())
                    .set(HELLO_WORLD, &mut conrod_ui);
            }));
        };
    }

    pub fn new() -> FixtureWindow {
        FixtureWindow {}
    }
}

// impl DMXWindow for FixtureWindow {
//     fn join(self) -> Result<(), Box<Any + Send + 'static>> {
//         self.thread.join()
//     }
// }

fn main() {
    FixtureWindow::start();
}
