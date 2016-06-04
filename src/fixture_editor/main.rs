#[macro_use] extern crate structures;
#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate piston_window;
extern crate rustc_serialize;

//use std::thread::{self, JoinHandle};
use conrod::{Canvas, Text, Button, Frameable, Colorable, Sizeable, Positionable, Widget, Labelable};
use piston_window::UpdateEvent;
use piston_window::Window;
//use std::any::Any;
use std::iter::*;
use std::fs;

use structures::ui::colors::FlatColor;
use structures::ui::window::{create_window};

mod file;
use file::*;

mod parser;
use parser::*;

widget_ids! {
    BACKGROUND,
    TITLE_BACKGROUND,
    TITLE,
    BODY,
    LEFT,
    RIGHT,
    CONFIG_SELECT_BUTTON with 100,
    CREATE_NEW_BUTTON,
    FIXTURE with 1000,
}

pub struct FixtureWindow {}

impl FixtureWindow {
    pub fn start() {
        let (mut window, mut conrod_ui) = match create_window("BitDMX Fixture Editor".to_string(), (711, 400), 30, true) {
            Ok(res) => res, Err(e) => {exit!(3, e);}
        };

        let mut config: Option<parser::Config> = None;


        while let Some(event) = window.next() {
            conrod_ui.handle_event(&event);
            window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

            event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {

                //Background
                Canvas::new()
                    .frame(0.0)
                    .pad(5.0)
                    .color(FlatColor::pickled_bluewood())
                    .w(window.size().width as f64)
                    .set(BACKGROUND, &mut conrod_ui);

                let mut title_size = 60.0;
                let mut show_title = true;

                if window.size().height  < 400 {
                    title_size = 20.0;
                    show_title = false;
                }

                Canvas::new()
                    .frame(0.0)
                    .pad(5.0)
                    .w(window.size().width as f64)
                    .h(title_size)
                    .align_top_of(BACKGROUND)
                    .color(FlatColor::ebony_clay())
                    .set(TITLE_BACKGROUND, &mut conrod_ui);

                if show_title {
                    Text::new("  BitDMX Fixture Editor")
                        .align_middle_y_of(TITLE_BACKGROUND)
                        .align_left_of(TITLE_BACKGROUND)
                        .font_size(30)
                        .color(FlatColor::clouds())
                        .set(TITLE, &mut conrod_ui);
                }

                Canvas::new()
                    .down_from(TITLE_BACKGROUND, 0.0)
                    .h(window.size().height as f64 - title_size)
                    .w(window.size().width as f64)
                    .color(FlatColor::pickled_bluewood())
                    .set(BODY, &mut conrod_ui);

                match config {
                    Some(ref mut config) => {
                        Canvas::new()
                            .frame(1.0)
                            .scroll_kids()
                            .pad(5.0)
                            .w_h(window.size().width as f64 / 2.0, window.size().height as f64 - title_size)
                            .top_left_of(BODY)
                            .color(FlatColor::pickled_bluewood())
                            .set(LEFT, &mut conrod_ui);

                        Canvas::new()
                            .frame(1.0)
                            .scroll_kids()
                            .pad(5.0)
                            .w_h(window.size().width as f64 / 2.0, window.size().height as f64 - title_size)
                            .right(0.0)
                            .color(FlatColor::pickled_bluewood())
                            .set(RIGHT, &mut conrod_ui)

                    },
                    None => {
                        Button::new()
                            .w_h(200.0, 30.0)
                            .color(FlatColor::silver())
                            .align_top_of(BODY)
                            .align_middle_x_of(BODY)
                            .label("Create new config")
                            .react(|| config = Some(parser::Config::new_empty()))
                            .set(CREATE_NEW_BUTTON, &mut conrod_ui);

                        let paths = fs::read_dir(file::get_path()).unwrap();

                        for (pathnumber, path) in paths.enumerate() {
                            Button::new()
                                .w_h(200.0, 30.0)
                                .color(FlatColor::silver())
                                .and_if(pathnumber > 0, |b| {
                                    b.down_from(CONFIG_SELECT_BUTTON + (pathnumber - 1), 10.0)
                                })
                                .and_if(pathnumber == 0, |b| {
                                    b.down_from(CREATE_NEW_BUTTON, 10.0)
                                })
                                .label(&path.unwrap().path().display().to_string())
                                .label_font_size(10)
                                .react(|| config = Some(parser::Config::new_empty()))
                                .set(CONFIG_SELECT_BUTTON + pathnumber, &mut conrod_ui);
                        }


                    },
                }
                
/*
                if file::check_for_file(path.clone()) == false {
                    Text::new(&"Couldn't find fixtures file".to_string())
                        .middle_of(BACKGROUND)
                        .font_size(15)
                        .color(FlatColor::clouds())
                        .set(FIXTURE + 0, &mut conrod_ui);

                    let file: parser::Config = parser::Config::new_empty();

                    file::write_file_content(path.clone(), parser::encode_file(file));
                } else {
                    Text::new(&file::get_file_content(path))
                        .middle_of(BACKGROUND)
                        .font_size(10)
                        .color(FlatColor::clouds())
                        .set(FIXTURE + 0, &mut conrod_ui);

                    let mut file: parser::Config = parser::parse_file();

                    for (fnumber, f) in file.fixture_templates.iter().enumerate() {
                        Button::new()
                            .w_h(200.0, 30.0)
                            .color(FlatColor::silver())
                            .down_from(FIXTURE + (fnumber), 10.0)
                            .label(&f.name)
                            .react(|| println!("Clicked!"))
                            .set(FIXTURE + fnumber + 1, &mut conrod_ui);
                    }

                }*/




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
