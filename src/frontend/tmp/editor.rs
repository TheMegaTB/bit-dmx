use std::sync::{Arc, Mutex};
use conrod::{
    color,
    Canvas,
    Colorable,
    Frameable,
    Positionable,
    Text,
    Widget,
    Button,
    DropDownList,
    Labelable,
    Sizeable,
    Slider,
    TextBox
};

use ui::UI;
use window::*;
use theme::*;
use structures::FadeTime;
use structures::FadeCurve;

#[allow(dead_code)]
widget_ids! {
    CANVAS,
    HEADER,
    BODY,
    CHASER_COLUMN,
    EDITOR_COLUMN,
    TITLE,
    CHASER_TITLE,
    CONNECTED_BUTTON,
    EDITOR_BUTTON,
    ADD_CHASER_BUTTON,
    EDITOR_TITLE,
    EDITOR_INFO,
    EDITOR_TIME_SLIDER,
    EDITOR_CHASER_TITLE with 4000,
    EDITOR_CONTENT with 4000,
    BUTTON with 4000,
    CONTROL_CHASER_TITLE with 4000,
    EDITOR_SWITCH_SLIDER with 4000,
    EDITOR_SWITCH_BUTTON with 4000,
    EDITOR_SWITCH_TEXT with 4000,
    EDITOR_SWITCH_DROP_DOWNS with 4000,
    EDITOR_CURVE_STRING1,
    EDITOR_CURVE_STRING2
}

pub fn draw_editor(mut conrod_ui: &mut UiCell, ui: &mut UI, application_theme: Theme) {
    let x_pos = 0.0;
    let mut y_pos = - 30.0 * application_theme.ui_scale;

    Text::new("Editor")
        .top_left_of(EDITOR_COLUMN)
        .font_size((22.0 * application_theme.ui_scale) as u32)
        .color(application_theme.bg_color.plain_contrast())
        .set(EDITOR_TITLE, conrod_ui);

    y_pos = y_pos - 40.0 * application_theme.ui_scale;

    let current_edited_switch = {
        ui.current_edited_switch_id.lock().unwrap()[0].clone()
    };

    let switch_name = ui.current_edited_switch_name.clone();

    match current_edited_switch {
        Some(switch_id) => {

            Text::new(&("Switch #".to_string() + &switch_id.to_string() + ": " + &ui.frontend_data.switches[switch_id].name.clone()))
                // .xy_relative_to(EDITOR_TITLE, [x_pos, y_pos])
                .align_middle_x()
                .down(60.0)
                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .color(application_theme.bg_color.plain_contrast())
                .set(EDITOR_INFO, conrod_ui);

            y_pos = y_pos - 60.0 * application_theme.ui_scale;

            let time = ui.frontend_data.switches[switch_id].before_chaser;
            let item_width = 320.0 * application_theme.ui_scale;
            let item_height = 40.0 * application_theme.ui_scale;
            let item_x_offset = 20.0 * application_theme.ui_scale;
            let line = "-----------------------------------------";
            let ref mut switch_name = switch_name.lock().unwrap()[0];
            //println!("name: {:?}", switch_name);


            TextBox::new(switch_name)
                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .w_h(item_width, item_height)
                .frame(2.0)
                .frame_color(application_theme.bg_color.invert().plain_contrast())
                .color(application_theme.bg_color.plain_contrast())
                .react(|new_name: &mut String| {
                    ui.frontend_data.switches[switch_id].name = new_name.clone();
                    ui.send_data();
                })
                .enabled(true)
                .set(EDITOR_CONTENT, conrod_ui);

            y_pos = y_pos - 60.0 * application_theme.ui_scale;

            let time_string = time.to_string();
            let label = {
                let mut text = "Chaser time: ".to_string();
                text.push_str(&time_string);
                text
            };

            Slider::new(time as f32, 0.0, 10000.0)
                .w_h(item_width, item_height)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.5, 0.3, 0.6)
                .frame(2.0)
                .label(&label)
                .label_color(color::WHITE)
                .react(|new_time: f32| {
                    ui.frontend_data.switches[switch_id].before_chaser = new_time as FadeTime;
                    ui.send_data();
                })
                .set(EDITOR_TIME_SLIDER, conrod_ui);

            let label = if ui.waiting_for_keybinding {
                "Keybinding: ?".to_string()
            }
            else {
                match ui.frontend_data.switches[switch_id].get_keybinding_as_text() {
                    Some(keybinding) => "Keybinding: ".to_string() + &keybinding,
                    None => "No keybinding".to_string()
                }
            };



            y_pos = y_pos - 60.0 * application_theme.ui_scale;
            let mut editor_switch_slider_count = 0;
            let mut editor_switch_button_count = 0;
            let mut editor_switch_text_count = 0;
            let mut editor_switch_drop_downs_count = 0;


            Button::new()
            .w_h(item_width, item_height)
            .xy_relative_to(TITLE, [x_pos, y_pos])
            .rgb(0.9, 0.9, 0.1)
            .frame(1.0)
            .label(&label)
            .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
            .react(|| {
                ui.waiting_for_keybinding = true;
            })
            .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
            editor_switch_button_count += 1;
            y_pos = y_pos - 60.0 * application_theme.ui_scale;

            Text::new(line)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .color(application_theme.bg_color.plain_contrast())
                .set(EDITOR_SWITCH_TEXT + editor_switch_text_count, conrod_ui);
            editor_switch_text_count += 1;
            y_pos = y_pos - 60.0 * application_theme.ui_scale;

            let cloned_ui = ui.clone();

            let mut data: Vec<String> = cloned_ui.frontend_data.switches[switch_id].channel_groups.keys().map(|x| x.clone()).collect();
            data.sort();

            //println!("{:?}", cloned_ui.frontend_data.switches[switch_id].channel_groups);

            let mut dropdown_list = Vec::new();
            let mut dropdown_background_list_fixture = Vec::new();
            let mut dropdown_background_list_channel_groups = Vec::new();
            for (fixture_index, fixture) in cloned_ui.frontend_data.fixtures.iter().enumerate() {
                if fixture.channel_groups.len() == 1 {
                    dropdown_list.push(fixture.name.clone());
                    dropdown_background_list_fixture.push(fixture_index);
                    dropdown_background_list_channel_groups.push(0);
                }
                else {
                    for (channel_group_index, _) in fixture.channel_groups.iter().enumerate() {
                        dropdown_list.push(fixture.name.clone() + ":" + &channel_group_index.to_string());
                        dropdown_background_list_fixture.push(fixture_index);
                        dropdown_background_list_channel_groups.push(channel_group_index);
                    }
                }
            }

            for (id_string, data) in data.iter().map(|x| (x, cloned_ui.frontend_data.switches[switch_id].channel_groups.get(x).unwrap())) {
                let mut id_vector: Vec<String> = id_string.split(",").map(|x| x.to_string()).collect();
                id_vector[0].remove(0);
                id_vector[1].pop();
                let fixture_id = id_vector[0].parse::<usize>().unwrap();
                let channel_group_id = id_vector[1].parse::<usize>().unwrap();



                let mut dropdown_index = 0;
                for (index, (&fixture_index, &channel_group_index)) in dropdown_background_list_fixture.iter().zip(dropdown_background_list_channel_groups.iter()).enumerate() {
                    if fixture_index == fixture_id && channel_group_index == channel_group_id {
                        dropdown_index = index;
                    }
                }

                DropDownList::new(&mut dropdown_list, &mut Some(dropdown_index))
                    .w_h(item_width-item_height, item_height)
                    .xy_relative_to(TITLE, [x_pos-item_height/2.0, y_pos])
                    .rgb(0.5, 0.3, 0.6)
                    .frame(2.0)
                    .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                    .label_color(color::WHITE)
                    .react(|_: &mut Option<usize>, new_idx, _: &str| {
                        if ui.frontend_data.change_channel_group(switch_id, id_string.clone(), dropdown_background_list_fixture[new_idx], dropdown_background_list_channel_groups[new_idx]) {
                            ui.current_edited_channel_group_id = new_idx as i64;
                            ui.send_data();
                        }
                        //println!("{:?}", ui.frontend_data.switches[switch_id].channel_groups);
                    })
                    .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                    editor_switch_drop_downs_count += 1;

                let label = if dropdown_index as i64 == ui.current_edited_channel_group_id {
                    "v".to_string()
                }
                else {
                    ">".to_string()
                };

                Button::new()
                    .w_h(item_height, item_height)
                    .xy_relative_to(TITLE, [x_pos+(item_width-item_height)/2.0, y_pos])
                    .rgb(0.9, 0.9, 0.1)
                    .frame(1.0)
                    .label(&label)
                    .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                    .react(|| {
                        if dropdown_index as i64 == ui.current_edited_channel_group_id {
                            ui.current_edited_channel_group_id = -1;
                        }
                        else {
                            ui.current_edited_channel_group_id = dropdown_index as i64;
                            let mut current_edited_curve_strings_locked = ui.current_edited_curve_strings.lock().unwrap();
                            current_edited_curve_strings_locked[0] = data.curve_in.get_string();
                            current_edited_curve_strings_locked[1] = data.curve_out.get_string();
                        };
                    })
                    .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                editor_switch_button_count += 1;
                y_pos = y_pos - 60.0 * application_theme.ui_scale;


                if dropdown_index as i64 == ui.current_edited_channel_group_id {

                    for (index, &value) in data.values.iter().enumerate() {
                        let label = {
                            let mut text = "Value: ".to_string();
                            text.push_str(&value.to_string());
                            text
                        };

                        Slider::new(value as f32, 0.0, 255.0)
                            .w_h(item_width - item_x_offset, item_height)
                            .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                            .rgb(0.5, 0.3, 0.6)
                            .frame(2.0)
                            .label(&label)
                            .label_color(color::WHITE)
                            .react(|new_value: f32| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().values[index] = new_value as u8;
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_SLIDER + editor_switch_slider_count, conrod_ui);
                        editor_switch_slider_count += 1;
                        y_pos = y_pos - 60.0 * application_theme.ui_scale;
                    }

                    let mut fade_curve_list = vec!("Linear".to_string(), "Squared".to_string(), "Square root".to_string(), "Custom".to_string());

                    {
                        let data = ui.frontend_data.switches[switch_id].channel_groups.get(id_string).unwrap().clone();

                        let fade_curve_id = data.curve_in.get_id();
                        DropDownList::new(&mut fade_curve_list, &mut Some(fade_curve_id))
                            .w_h(item_width - item_x_offset, item_height)
                            .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                            .rgb(0.5, 0.3, 0.6)
                            .frame(2.0)
                            .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                            .label_color(color::WHITE)
                            .react(|_: &mut Option<usize>, new_idx, _: &str| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::get_by_id(new_idx, "x".to_string());
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                        editor_switch_drop_downs_count += 1;
                        y_pos = y_pos - 60.0 * application_theme.ui_scale;

                        if fade_curve_id == 3 {
                            let ref mut curve_string = {ui.current_edited_curve_strings.lock().unwrap()[0].clone()};

                            TextBox::new(curve_string)
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                                .frame(2.0)
                                .frame_color(application_theme.bg_color.invert().plain_contrast())
                                .color(application_theme.bg_color.plain_contrast())
                                .react(|new_name: &mut String| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::Custom(new_name.clone());
                                    ui.send_data();
                                })
                                .enabled(true)
                                .set(EDITOR_CURVE_STRING1, conrod_ui);
                            y_pos = y_pos - 60.0 * application_theme.ui_scale;
                        }

                        let label = {
                            let mut text = "Time in: ".to_string();
                            text.push_str(&data.time_in.to_string());
                            text
                        };

                        Slider::new(data.time_in as f32, 0.0, 10000.0)
                            .w_h(item_width - item_x_offset, item_height)
                            .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                            .rgb(0.5, 0.3, 0.6)
                            .frame(2.0)
                            .label(&label)
                            .label_color(color::WHITE)
                            .react(|new_value: f32| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().time_in = new_value as FadeTime;
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_SLIDER + editor_switch_slider_count, conrod_ui);
                        editor_switch_slider_count += 1;
                        y_pos = y_pos - 60.0 * application_theme.ui_scale;

                        let fade_curve_id = data.curve_out.get_id();
                        DropDownList::new(&mut fade_curve_list, &mut Some(fade_curve_id))
                            .w_h(item_width - item_x_offset, item_height)
                            .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                            .rgb(0.5, 0.3, 0.6)
                            .frame(2.0)
                            .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                            .label_color(color::WHITE)
                            .react(|_: &mut Option<usize>, new_idx, _: &str| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::get_by_id(new_idx, "x".to_string());
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                        editor_switch_drop_downs_count += 1;
                        y_pos = y_pos - 60.0 * application_theme.ui_scale;

                        if fade_curve_id == 3 {
                            let ref mut curve_string = {ui.current_edited_curve_strings.lock().unwrap()[1].clone()};

                            TextBox::new(curve_string)
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .frame(2.0)
                                .frame_color(application_theme.bg_color.invert().plain_contrast())
                                .color(application_theme.bg_color.plain_contrast())
                                .react(|new_name: &mut String| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::Custom(new_name.clone());
                                    ui.send_data();
                                })
                                .enabled(true)
                                .set(EDITOR_CURVE_STRING2, conrod_ui);
                            y_pos = y_pos - 60.0 * application_theme.ui_scale;
                        }

                        let label = {
                            let mut text = "Time out: ".to_string();
                            text.push_str(&data.time_out.to_string());
                            text
                        };

                        Slider::new(data.time_out as f32, 0.0, 10000.0)
                            .w_h(item_width - item_x_offset, item_height)
                            .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                            .rgb(0.5, 0.3, 0.6)
                            .frame(2.0)
                            .label(&label)
                            .label_color(color::WHITE)
                            .react(|new_value: f32| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().time_out = new_value as FadeTime;
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_SLIDER + editor_switch_slider_count, conrod_ui);
                        editor_switch_slider_count += 1;
                        y_pos = y_pos - 60.0 * application_theme.ui_scale;
                    }

                    Button::new()
                        .w_h(item_width - item_x_offset, item_height)
                        .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                        .rgb(0.9, 0.1, 0.1)
                        .frame(1.0)
                        .label("Delete")
                        .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                        .react(|| {
                            ui.frontend_data.remove_channel_group(switch_id, id_string.clone());
                            ui.send_data();
                        })
                        .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                        editor_switch_button_count += 1;
                    y_pos = y_pos - 60.0 * application_theme.ui_scale;
                }
            }
            Button::new()
                .w_h(item_width, item_height)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.1, 0.9, 0.1)
                .frame(1.0)
                .label("Add")
                .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .react(|| {
                    ui.frontend_data.add_channel_group(switch_id);
                    ui.send_data();
                })
                .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
            editor_switch_button_count += 1;
            y_pos = y_pos - 60.0 * application_theme.ui_scale;

            Button::new()
                .w_h(item_width, item_height)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.9, 0.1, 0.1)
                .frame(1.0)
                .label("Delete")
                .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .react(|| {
                    ui.frontend_data.remove_switch_with_id(switch_id);
                    ui.current_edited_switch_id.lock().unwrap()[0] = None;
                    ui.send_data();
                })
                .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
        }
        None => {
            Text::new("No switch selected")
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .color(application_theme.bg_color.plain_contrast())
                .set(EDITOR_INFO, conrod_ui);
        }
    }
}
