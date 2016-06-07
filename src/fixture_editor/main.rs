#[macro_use] extern crate structures;
#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate piston_window;
extern crate rustc_serialize;

use conrod::{Canvas, Text, Button, Frameable, Colorable, Sizeable, Positionable, Widget, Labelable, TextBox, DropDownList, Slider};
use piston_window::UpdateEvent;
use piston_window::Window;
use std::iter::*;
use std::fs;

use structures::logic::channel::DmxAddress;

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
    SAVE_BUTTON,
    CLOSE_BUTTON,
    BODY,
    LEFT,
    RIGHT,
    CONFIG_SELECT_BUTTON with 100,
    CREATE_NEW_BUTTON,
    FIXTURE with 1000,
    CHANNEL_GROUP with 256,
    ADD_FIXTURE_TEMPLATE_BUTTON,
    ADD_CHANNEL_GROUP_BUTTON,
    BACK_TO_FIXTURE_TEMPLATE_LIST_BUTTON,
    RENAME_TEXTBOX,
    CHOOSE_CHANNEL_GROUP,
    BACK_TO_CHANNEL_GROUP_LIST_BUTTON,
    DELETE_CHANNEL_GROUP_BUTTON,
    CHANNEL_SLIDER with 256,
    DELETE_FIXTURE_TEMPLATE_BUTTON,
    RENAME_PROJECT_TEXTBOX
}

pub struct FixtureWindow {}

impl FixtureWindow {
    pub fn start() {
        let (mut window, mut conrod_ui) = match create_window("BitDMX Fixture Editor".to_string(), (711, 400), 30, true) {
            Ok(res) => res, Err(e) => {exit!(3, e);}
        };

        let mut config: Option<parser::Config> = None;
        let mut compare_config: String = "".to_string();
        let mut path_to_config = "".to_string();
        let mut config_name = "".to_string();
        let mut config_names: Vec<String> = Vec::new();
        let mut save_color = FlatColor::silver();
        let mut changed_counter = 0;

        let mut close = false;
        let mut reset_config = false;

        let mut current_fixture_templat_id: Option<usize> = None;
        let mut current_channel_group_id: Option<usize> = None;

        let mut channel_group_type = 0;
        let mut channel_group_values: Vec<DmxAddress> = vec!(0);

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

                //Close
                if close == true {
                    if save_color == FlatColor::silver() {
                        reset_config = true;
                        close = false;
                    } else {
                        Text::new("There are unsaved changes.\nWould you like to save them?")
                            .middle_of(BACKGROUND)
                            .font_size(30)
                            .color(FlatColor::clouds())
                            .set(TITLE, &mut conrod_ui);

                        //Save Button
                        Button::new()
                            .w_h(200.0, 40.0)
                            .color(FlatColor::emerald())
                            .down_from(TITLE, 5.0)
                            .label("Yes")
                            .label_font_size(30)
                            .react(|| {
                                file::write_file_content(path_to_config.clone(), parser::encode_file(config.clone().unwrap()));
                                compare_config = parser::encode_file(config.clone().unwrap());
                                close = false;
                                reset_config = true;
                            })
                            .set(SAVE_BUTTON, &mut conrod_ui);

                        //Close Button
                        Button::new()
                            .w_h(200.0, 40.0)
                            .color(FlatColor::alizarin())
                            .right_from(SAVE_BUTTON, 50.0)
                            .label("No")
                            .label_font_size(30)
                            .react(|| {
                                reset_config = true;
                                close = false;
                            })
                            .set(CLOSE_BUTTON, &mut conrod_ui);
                    }
                } else {
                    let mut title_size = 70.0;
                    let mut show_title = true;

                    if window.size().height  < 400 {
                        title_size = 20.0;
                        show_title = false;
                    }

                    //Title Background
                    Canvas::new()
                        .frame(0.0)
                        .pad(5.0)
                        .w(window.size().width as f64)
                        .h(title_size)
                        .align_top_of(BACKGROUND)
                        .color(FlatColor::ebony_clay())
                        .set(TITLE_BACKGROUND, &mut conrod_ui);

                    //Title
                    if show_title {
                        Text::new("  BitDMX Fixture Editor")
                            .align_middle_y_of(TITLE_BACKGROUND)
                            .align_left_of(TITLE_BACKGROUND)
                            .font_size(30)
                            .color(FlatColor::clouds())
                            .set(TITLE, &mut conrod_ui);
                    }

                    //Body Canvas
                    Canvas::new()
                        .down_from(TITLE_BACKGROUND, 0.0)
                        .h(window.size().height as f64 - title_size)
                        .w(window.size().width as f64)
                        .color(FlatColor::pickled_bluewood())
                        .set(BODY, &mut conrod_ui);

                    match config {
                        Some(ref mut config) => {

                            //Save Button
                            Button::new()
                                .w_h(100.0, 20.0)
                                .color(save_color)
                                .top_right_with_margin_on(TITLE_BACKGROUND, 5.0)
                                .label("Save")
                                .label_font_size(15)
                                .react(|| {
                                    file::write_file_content(path_to_config.clone(), parser::encode_file(config.clone()));
                                    compare_config = parser::encode_file(config.clone());
                                })
                                .set(SAVE_BUTTON, &mut conrod_ui);

                            //Close Button
                            Button::new()
                                .w_h(100.0, 20.0)
                                .color(FlatColor::alizarin())
                                .down_from(SAVE_BUTTON, 10.0)
                                .label("Close")
                                .label_font_size(15)
                                .react(|| {
                                    close = true;
                                })
                                .set(CLOSE_BUTTON, &mut conrod_ui);

                            //Project Name TextBox
                            TextBox::new(&mut config_name)
                                .enabled(true)
                                .mid_right_with_margin_on(TITLE_BACKGROUND, 110.0)
                                .h(50.0)
                                .w(150.0)
                                .font_size(20)
                                .react(|_: &mut String| {})
                                .set(RENAME_PROJECT_TEXTBOX, &mut conrod_ui);

                            ///////////////
                            // Left Side //
                            ///////////////
                            Canvas::new()
                                .frame(1.0)
                                .scroll_kids_vertically() //TODO
                                .pad(5.0)
                                .w_h(window.size().width as f64 / 2.0, window.size().height as f64 - title_size)
                                .top_left_of(BODY)
                                .color(FlatColor::pickled_bluewood())
                                .set(LEFT, &mut conrod_ui);

                            match current_fixture_templat_id {
                                Some(id) => {
                                    match current_channel_group_id {
                                        Some(cg_id) => {
                                            let mut dropdown_list: Vec<String> = vec!("Single".to_string(), "RGB".to_string(), "RGBA".to_string(), "Moving2D".to_string());

                                            DropDownList::new(&mut dropdown_list, &mut Some(channel_group_type))
                                                .react(|_: &mut Option<usize>, cgt, _: &str| {
                                                    channel_group_type = cgt;
                                                    match cgt {
                                                        0 => channel_group_values = vec!(0),
                                                        1 => channel_group_values = vec!(0, 1, 2),
                                                        2 => channel_group_values = vec!(0, 1, 2, 3),
                                                        3 => channel_group_values = vec!(0, 1),
                                                        _=> channel_group_values = vec!(0),
                                                    }
                                                })
                                                .top_left_with_margin_on(LEFT, 10.0)
                                                .w_h((window.size().width as f64  / 2.0) - 20.0, 40.0)
                                                .set(CHOOSE_CHANNEL_GROUP, &mut conrod_ui);

                                            //Back to Fixture Template List
                                            Button::new()
                                                .h(30.0)
                                                .and_if(window.size().width > 799, |b| {
                                                    b.w(200.0)
                                                })
                                                .and_if(window.size().width < 800, |b| {
                                                    b.w(window.size().width as f64 / 4.0 - 20.0)
                                                })
                                                .label("Back")
                                                .down_from(RENAME_TEXTBOX, 10.0)
                                                .color(FlatColor::amethyst())
                                                .react(|| {
                                                    current_channel_group_id = None;

                                                    let channel_group = match channel_group_type {
                                                        0 => parser::ChannelGroup::Single(channel_group_values[0]),
                                                        1 => parser::ChannelGroup::RGB(channel_group_values[0], channel_group_values[1], channel_group_values[2]),
                                                        2 => parser::ChannelGroup::RGBA(channel_group_values[0], channel_group_values[1], channel_group_values[2], channel_group_values[3]),
                                                        3 => parser::ChannelGroup::Moving2D(channel_group_values[0], channel_group_values[1]),
                                                        _ => parser::ChannelGroup::Single(channel_group_values[0]),
                                                    };
                                                    config.fixture_templates[id].channel_groups[cg_id] = channel_group;
                                                })
                                                .set(BACK_TO_CHANNEL_GROUP_LIST_BUTTON, &mut conrod_ui);

                                            //Delete
                                            Button::new()
                                                .h(30.0)
                                                .and_if(window.size().width > 799, |b| {
                                                    b.w(200.0)
                                                })
                                                .and_if(window.size().width < 800, |b| {
                                                    b.w(window.size().width as f64 / 4.0 - 20.0)
                                                })
                                                .label("Delete")
                                                .right_from(BACK_TO_CHANNEL_GROUP_LIST_BUTTON, 10.0)
                                                .color(FlatColor::alizarin())
                                                .react(|| {
                                                    current_channel_group_id = None;
                                                    config.fixture_templates[id].channel_groups.remove(cg_id);
                                                })
                                                .set(DELETE_CHANNEL_GROUP_BUTTON, &mut conrod_ui);

                                            //Sliders
                                            for (count, cgv) in channel_group_values.clone().iter_mut().enumerate() {
                                                Slider::new(*cgv as f32, 0.0, 255.0)
                                                    .h(30.0)
                                                    .and_if(window.size().width > 399, |b| {
                                                        b.w(200.0)
                                                    })
                                                    .and_if(window.size().width < 400, |b| {
                                                        b.w(window.size().width as f64 / 2.0 - 20.0)
                                                    })
                                                    .and_if(count > 0, |b| {
                                                        b.down_from(CHANNEL_SLIDER + (count - 1), 10.0)
                                                    })
                                                    .and_if(count == 0, |b| {
                                                        b.down_from(BACK_TO_CHANNEL_GROUP_LIST_BUTTON, 10.0)
                                                    })
                                                    .color(FlatColor::clouds())
                                                    .skew(1.0)
                                                    .react(|value: f32| channel_group_values[count] = value as DmxAddress)
                                                    .set(CHANNEL_SLIDER + count, &mut conrod_ui);
                                            }
                                        }
                                        None => {
                                            //Fixture Template Name TextBox
                                            TextBox::new(&mut config.fixture_templates[id].name)
                                                .enabled(true)
                                                .top_left_with_margin_on(LEFT, 10.0)
                                                .w_h((window.size().width as f64  / 2.0) - 20.0, 40.0)
                                                .font_size(30)
                                                .react(|_: &mut String| {})
                                                .set(RENAME_TEXTBOX, &mut conrod_ui);

                                            //Back to Fixture Template List
                                            Button::new()
                                                .h(30.0)
                                                .and_if(window.size().width > 799, |b| {
                                                    b.w(200.0)
                                                })
                                                .and_if(window.size().width < 800, |b| {
                                                    b.w(window.size().width as f64 / 4.0 - 20.0)
                                                })
                                                .label("Back")
                                                .down_from(RENAME_TEXTBOX, 10.0)
                                                .color(FlatColor::amethyst())
                                                .react(|| current_fixture_templat_id = None)
                                                .set(BACK_TO_FIXTURE_TEMPLATE_LIST_BUTTON, &mut conrod_ui);

                                            //New Group Button
                                            Button::new()
                                                .h(30.0)
                                                .and_if(window.size().width > 799, |b| {
                                                    b.w(200.0)
                                                })
                                                .and_if(window.size().width < 800, |b| {
                                                    b.w(window.size().width as f64 / 4.0 - 20.0)
                                                })
                                                .label("New Group")
                                                .down_from(BACK_TO_FIXTURE_TEMPLATE_LIST_BUTTON, 10.0)
                                                .color(FlatColor::emerald())
                                                .react(|| config.fixture_templates[id].channel_groups.push(parser::ChannelGroup::Single(0)))
                                                .set(ADD_CHANNEL_GROUP_BUTTON, &mut conrod_ui);

                                            //ChannelGroup List
                                            for (count, channel_group) in config.fixture_templates[id].channel_groups.iter().enumerate() {

                                                let (name, channels, channel_group_type_id) = match channel_group {
                                                    &parser::ChannelGroup::Single(ch1) => ("Single", vec!(ch1), 0),
                                                    &parser::ChannelGroup::RGB(ch1, ch2, ch3) => ("RGB", vec!(ch1, ch2, ch3), 1),
                                                    &parser::ChannelGroup::RGBA(ch1, ch2, ch3, ch4) => ("RGBA", vec!(ch1, ch2, ch3, ch4), 2),
                                                    &parser::ChannelGroup::Moving2D(ch1, ch2) => ("Moving2D", vec!(ch1, ch2), 3),
                                                };

                                                Button::new()
                                                    .h(30.0)
                                                    .and_if(window.size().width > 399, |b| {
                                                        b.w(200.0)
                                                    })
                                                    .and_if(window.size().width < 400, |b| {
                                                        b.w(window.size().width as f64 / 2.0 - 20.0)
                                                    })
                                                    .label(name)
                                                    .and_if(count > 0, |b| {
                                                        b.down_from(CHANNEL_GROUP + (count - 1), 10.0)
                                                    })
                                                    .and_if(count == 0, |b| {
                                                        b.down_from(ADD_CHANNEL_GROUP_BUTTON, 10.0)
                                                    })
                                                    .color(FlatColor::clouds())
                                                    .react(|| {
                                                        current_channel_group_id = Some(count);
                                                        channel_group_type = channel_group_type_id;
                                                        channel_group_values = channels;
                                                    })
                                                    .set(CHANNEL_GROUP + count, &mut conrod_ui);
                                            }

                                            //Delete
                                            Button::new()
                                                .h(30.0)
                                                .and_if(window.size().width > 799, |b| {
                                                    b.w(200.0)
                                                })
                                                .and_if(window.size().width < 800, |b| {
                                                    b.w(window.size().width as f64 / 4.0 - 20.0)
                                                })
                                                .label("Delete")
                                                .right_from(BACK_TO_FIXTURE_TEMPLATE_LIST_BUTTON, 10.0)
                                                .color(FlatColor::alizarin())
                                                .react(|| {
                                                    config.fixture_templates.remove(id);
                                                    current_fixture_templat_id = None;
                                                })
                                                .set(DELETE_FIXTURE_TEMPLATE_BUTTON, &mut conrod_ui);
                                        }
                                    }
                                }
                                None => {
                                    //New Template Button
                                    Button::new()
                                        .h(30.0)
                                        .and_if(window.size().width > 399, |b| {
                                            b.w(200.0)
                                        })
                                        .and_if(window.size().width < 400, |b| {
                                            b.w(window.size().width as f64 / 2.0 - 20.0)
                                        })
                                        .label("New Template")
                                        .top_left_with_margin_on(LEFT, 10.0)
                                        .color(FlatColor::emerald())
                                        .react(|| config.fixture_templates.push(parser::FixtureTemplate::new_empty()))
                                        .set(ADD_FIXTURE_TEMPLATE_BUTTON, &mut conrod_ui);

                                    //Template List
                                    for (count, fixture_template) in config.fixture_templates.iter().enumerate() {
                                        Button::new()
                                            .h(30.0)
                                            .and_if(window.size().width > 399, |b| {
                                                b.w(200.0)
                                            })
                                            .and_if(window.size().width < 400, |b| {
                                                b.w(window.size().width as f64 / 2.0 - 20.0)
                                            })
                                            .label(&fixture_template.name)
                                            .and_if(count > 0, |b| {
                                                b.down_from(FIXTURE + (count - 1), 10.0)
                                            })
                                            .and_if(count == 0, |b| {
                                                b.down_from(ADD_FIXTURE_TEMPLATE_BUTTON, 10.0)
                                            })
                                            .color(FlatColor::clouds())
                                            .react(|| current_fixture_templat_id = Some(count))
                                            .set(FIXTURE + count, &mut conrod_ui);
                                    }
                                }
                            }

                            ////////////////
                            // Right Side //
                            ////////////////
                            Canvas::new()
                                .frame(1.0)
                                .scroll_kids()
                                .pad(5.0)
                                .w_h(window.size().width as f64 / 2.0, window.size().height as f64 - title_size)
                                .align_right_of(BODY)
                                .color(FlatColor::pickled_bluewood())
                                .set(RIGHT, &mut conrod_ui);


                            changed_counter = changed_counter + 1;
                            if changed_counter > 29 {
                                changed_counter = 0;

                                if parser::encode_file(config.clone()) == compare_config {
                                    save_color = FlatColor::silver();
                                } else {
                                    save_color = FlatColor::emerald();
                                }
                            }
                        },
                        None => {
                            Button::new()
                                .w_h(200.0, 30.0)
                                .color(FlatColor::silver())
                                .mid_top_with_margin_on(BODY, 10.0)
                                .label("Create new config")
                                .label_font_size(15)
                                .react(|| config = Some(parser::Config::new_empty()))
                                .set(CREATE_NEW_BUTTON, &mut conrod_ui);

                            let paths = fs::read_dir(file::get_path()).unwrap();

                            for (pathnumber, path) in paths.enumerate() {
                                config_names.push(path.unwrap().path().file_name().unwrap().to_str().unwrap().clone().to_string());

                                Button::new()
                                    .w_h(200.0, 30.0)
                                    .color(FlatColor::silver())
                                    .and_if(pathnumber > 0, |b| {
                                        b.down_from(CONFIG_SELECT_BUTTON + (pathnumber - 1), 10.0)
                                    })
                                    .and_if(pathnumber == 0, |b| {
                                        b.down_from(CREATE_NEW_BUTTON, 10.0)
                                    })
                                    .label(&config_names[pathnumber].clone())
                                    .label_font_size(15)
                                    .react(|| {
                                        config_name = config_names[pathnumber].clone();
                                        path_to_config = fs::read_dir(file::get_path()).unwrap().nth(pathnumber).unwrap().unwrap().path().display().to_string().clone() + "/fixtures.dmx";
                                        if file::check_for_file(path_to_config.clone()) {
                                            match parser::decode_file(file::get_file_content(path_to_config.clone())) {
                                                Some(c) => {
                                                    config = Some(c);
                                                    compare_config = parser::encode_file(config.clone().unwrap());
                                                },
                                                _ => println!("There has been an error parsing the file.\nTry an other file or create a new one.")
                                            }
                                        } else {
                                            path_to_config = "".to_string();
                                        }
                                    })
                                    .set(CONFIG_SELECT_BUTTON + pathnumber, &mut conrod_ui);
                            }
                        },
                    }
                }
                if reset_config {
                    config = None;
                    reset_config = false;
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
