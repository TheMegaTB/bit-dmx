#[macro_use] extern crate structures;
#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate piston_window;

use std::thread::{self, JoinHandle};
use conrod::{Canvas, Text, Frameable, Colorable, Sizeable, Positionable, Widget};
use piston_window::UpdateEvent;
use piston_window::Window;
use std::any::Any;

use structures::ui::colors::FlatColor;
use structures::ui::window::{create_window, DMXWindow};

mod file;
use file::*;

widget_ids! {
    BACKGROUND,
    TITLE_BACKGROUND,
    TITLE,
    HELLO_WORLD
}

pub struct FixtureWindow {}

impl FixtureWindow {
    pub fn start() {
        let (mut window, mut conrod_ui) = match create_window("BitDMX Fixture Editor".to_string(), (711, 400), 30, true) {
            Ok(res) => res, Err(e) => {exit!(3, e);}
        };

        while let Some(event) = window.next() {
            conrod_ui.handle_event(&event);
            window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

            event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {

                //Background
                Canvas::new()
                    .frame(0.0)
                    .pad(5.0)
                    .color(FlatColor::pickled_bluewood())
                    .set(BACKGROUND, &mut conrod_ui);

                //Title Bar
                if(window.size().height  < 400) {
                    Canvas::new()
                        .frame(0.0)
                        .pad(5.0)
                        .w(window.size().width as f64)
                        .h(20.0)
                        .align_top_of(BACKGROUND)
                        .color(FlatColor::ebony_clay())
                        .set(TITLE_BACKGROUND, &mut conrod_ui);
                } else {
                    Canvas::new()
                        .frame(0.0)
                        .pad(5.0)
                        .w(window.size().width as f64)
                        .h(60.0)
                        .align_top_of(BACKGROUND)
                        .color(FlatColor::ebony_clay())
                        .set(TITLE_BACKGROUND, &mut conrod_ui);

                    Text::new("  BitDMX Fixture Editor")
                        .align_middle_y_of(TITLE_BACKGROUND)
                        .align_left_of(TITLE_BACKGROUND)
                        .font_size(30)
                        .color(FlatColor::clouds())
                        .set(TITLE, &mut conrod_ui);
                }

                let mut path: String = file::get_path().clone();

                if(file::check_for_file(path.clone()) == false) {
                    Text::new(&"Couldn't find fixtures file".to_string())
                        .middle_of(BACKGROUND)
                        .font_size(15)
                        .color(FlatColor::clouds())
                        .set(HELLO_WORLD, &mut conrod_ui);
                } else {
                    Text::new(&file::get_file_content(path))
                        .middle_of(BACKGROUND)
                        .font_size(10)
                        .color(FlatColor::clouds())
                        .set(HELLO_WORLD, &mut conrod_ui);
                }




            }));
        };
    }

    pub fn new() -> FixtureWindow {
        FixtureWindow {}
    }
}

fn main() {
    FixtureWindow::start();
}
