#[macro_use] extern crate log;
#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate find_folder;
#[macro_use] extern crate structures;
extern crate rustc_serialize;
use structures::*;

use std::cmp::max;
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
    NumberDialer,
    TextBox
};
use piston_window::{ UpdateEvent, PressEvent, ReleaseEvent, Window };

mod colors;

mod ui;
use ui::UI;

mod window;
use window::*;

mod splash;
use splash::*;

mod theme;
use theme::*;

mod frontend_helpers;
use frontend_helpers::*;

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
    EDITOR_TIME_NUMBER_DIALER,
    EDITOR_CHASER_TITLE with 4000,
    EDITOR_CONTENT with 4000,
    BUTTON with 4000,
    CONTROL_CHASER_TITLE with 4000,
    EDITOR_SWITCH_SLIDER with 4000,
    EDITOR_SWITCH_NUMBER_DIALER with 4000,
    EDITOR_SWITCH_BUTTON with 4000,
    EDITOR_SWITCH_TEXT with 4000,
    EDITOR_SWITCH_DROP_DOWNS with 4000,
    EDITOR_CURVE_STRING1,
    EDITOR_CURVE_STRING2
}

fn create_output_window(ui: Arc<Mutex<UI>>) {
    let (mut window, mut conrod_ui) = match create_window("Sushi Reloaded!".to_string(), (1100, 560), 30, false) {
        Ok(res) => res,
        Err(e) => {
            exit!(3, e);
        }
    };

    let app_theme = Theme::default();

    // Poll events from the window.
    let mut button_pressed = false;
    while let Some(event) = window.next() {
        let mut ui_locked = ui.lock().expect("Failed to lock Arc!");

        // Button/Mouse events
        if let Some(button) = event.press_args() {
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift) {
                ui_locked.shift_state = true;
            }
            else if button == piston_window::Button::Keyboard(piston_window::Key::LCtrl) {
                ui_locked.control_state = true;
            }
            if ui_locked.waiting_for_keybinding {
                let switch_id = ui_locked.current_edited_switch_id.lock().expect("Failed to lock Arc!")[0];
                match switch_id {
                    Some(switch_id) => {
                        match button {
                            piston_window::Button::Keyboard(key) =>  {
                                ui_locked.frontend_data.switches[switch_id].keybinding = Some(key);
                                ui_locked.waiting_for_keybinding = false;
                                ui_locked.send_data();
                            },
                            _ => {
                                ui_locked.frontend_data.switches[switch_id].keybinding = None;
                                ui_locked.waiting_for_keybinding = false;
                                ui_locked.send_data();
                            }
                        }
                    },
                    None => {}
                }
            }
            else if !ui_locked.edit_state {
                match button {
                    piston_window::Button::Keyboard(key) =>  {
                        for (switch_id, switch) in ui_locked.frontend_data.switches.iter().enumerate() {
                            if switch.keybinding == Some(key) {
                                let new_value = if switch.dimmer_value == 0.0 {255} else {0};
                                ui_locked.tx.send(get_switch_update(ui_locked.shift_state, switch_id as u16, new_value)).unwrap();
                            }
                        }
                    },
                    _ => {}
                }
            }
            else {
                button_pressed = true;
            }
        }
        else if let Some(button) = event.release_args() {
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift) {
                ui_locked.shift_state = false;
            }
            else if button == piston_window::Button::Keyboard(piston_window::Key::LCtrl) {
                ui_locked.control_state = false;
            }
        }

        //Drawing
        conrod_ui.handle_event(&event);
        event.update(|_| {
            conrod_ui.set_widgets(|mut conrod_ui| set_widgets(&mut conrod_ui, &mut ui_locked, app_theme.clone(), window.size().width as f64, button_pressed));
            button_pressed = false;
        });
        window.draw_2d(&event, |c, g| conrod_ui.draw_if_changed(c, g));
    }
}


fn set_widgets(mut conrod_ui: &mut UiCell, ui: &mut UI, app_theme: Theme, window_width: f64, button_pressed: bool) {

    let editor_width = if ui.edit_state {
        (window_width/3.0).min(350.0)
    }
    else {
        0.0
    };

    let header = Canvas::new().color(app_theme.bg_header).pad(app_theme.ui_padding).length(100.0 * app_theme.ui_scale);
    let control_column = Canvas::new().color(app_theme.bg_control).scroll_kids().pad(app_theme.ui_padding);
    let editor_column = Canvas::new().color(app_theme.bg_editor).scroll_kids().pad(app_theme.ui_padding).length(editor_width);

    Canvas::new()
        .frame(1.0)
        .color(app_theme.bg_header)
        .flow_down(&[
             (HEADER, header),
             (BODY, Canvas::new().flow_right(&[
                 (CHASER_COLUMN, control_column),
                 (EDITOR_COLUMN, editor_column)
             ]))
          ])
        .set(CANVAS, &mut conrod_ui);

    // println!("{:?}", button_pressed);
    draw_header(conrod_ui, ui, app_theme.clone());

    let chasers_usable_width = window_width-editor_width-2.0*app_theme.ui_padding;
    draw_chasers(conrod_ui, ui, app_theme.clone(), chasers_usable_width, button_pressed);

    if ui.edit_state {
        draw_editor(conrod_ui, ui, app_theme.clone(), editor_width, button_pressed);
    }
}

fn draw_header(mut conrod_ui: &mut UiCell, ui: &mut UI, app_theme: Theme) {
    Text::new("Moonshadow 2016!")
        .top_left_of(HEADER)
        .font_size((32.0 * app_theme.ui_scale) as u32)
        .color(app_theme.bg_header.plain_contrast())
        .set(TITLE, conrod_ui);

    let connected = ui.watchdog.is_alive();
    let label = if connected { "Connected".to_string() } else { "Disconnected".to_string() };
    Button::new()
        .w_h(140.0 * app_theme.ui_scale, 35.0 * app_theme.ui_scale)
        .down_from(TITLE, 7.0 * app_theme.ui_scale)
        .frame(1.0)
        .label(&label)
        .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
        .and(|b| {
            if connected {
                b.color(app_theme.switch_on_color)
            } else {
                b.color(app_theme.switch_off_color)
            }
        })
        .react(|| {})
        .set(CONNECTED_BUTTON, conrod_ui);

    Button::new()
        .w_h(140.0 * app_theme.ui_scale, 35.0 * app_theme.ui_scale)
        .right_from(CONNECTED_BUTTON, 5.0 * app_theme.ui_scale)
        .frame(1.0)
        .label(&"Edit Mode".to_string())
        .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
        .and(|b| {
            if ui.edit_state {
                b.color(app_theme.switch_on_color)
            } else {
                b.color(app_theme.switch_off_color)
            }
        })
        .react(|| {
            ui.current_edited_switch_id.lock().expect("Failed to lock Arc!")[0] = None;
            ui.current_edited_switch_name.lock().expect("Failed to lock Arc!")[0] = "".to_string();
            if ui.edit_state {
                ui.send_data();
            }
            else {
                ui.current_edited_chaser_names = Arc::new(Mutex::new(ui.chasers.clone()));
            }
            ui.edit_state = !ui.edit_state;
        })
        .set(EDITOR_BUTTON, conrod_ui);

    if ui.edit_state {
        Button::new()
            .w_h(140.0 * app_theme.ui_scale, 35.0 * app_theme.ui_scale)
            .right_from(EDITOR_BUTTON, 5.0 * app_theme.ui_scale)
            .frame(1.0)
            .label(&"Add Chaser".to_string())
            .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
            .color(app_theme.switch_on_color)
            .react(|| {
                let name = ui.frontend_data.add_chaser();
                ui.chasers.push(name);
                ui.current_edited_chaser_names = Arc::new(Mutex::new(ui.chasers.clone()));
                ui.save_chaser_config();
                ui.send_data();
            })
            .set(ADD_CHASER_BUTTON, conrod_ui);
    }
}

fn draw_chasers(mut conrod_ui: &mut UiCell, ui: &mut UI, app_theme: Theme, usable_width: f64, button_pressed: bool) {
    let tx = ui.tx.clone();

    let chasers = {
        let mut tmp_chasers = Vec::new();
        for chaser_name in ui.chasers.iter() {
            if ui.frontend_data.chasers.contains_key(chaser_name) {
                tmp_chasers.push(chaser_name.clone());
            }
        }
        tmp_chasers
    };

    const TEXT_BLOCK_WIDTH: f64 = 100.0;
    Text::new("Chaser")
        .w_h(TEXT_BLOCK_WIDTH, 20.0)
        .align_text_left()
        .top_left_of(CHASER_COLUMN)
        .font_size((22.0 * app_theme.ui_scale) as u32)
        .color(app_theme.bg_control.plain_contrast())
        .set(CHASER_TITLE, conrod_ui);

    let original_button_width = 200.0 * app_theme.ui_scale;
    let chasers_per_row = max((usable_width/original_button_width) as u8, 1);
    let button_width = usable_width/chasers_per_row as f64;
    let button_height = 50.0 * app_theme.ui_scale;
    let mut current_button_id = BUTTON;

    let mut next_y_offset = 0f64;
    let x_offset = button_width/2.0 - TEXT_BLOCK_WIDTH/2.0;
    let mut column = 0.0;
    let mut y_offset = -50.0 * app_theme.ui_scale;

    let cloned_ui = ui.clone();




    for (id, (name, chaser)) in chasers.iter().map(|x| (x, cloned_ui.frontend_data.chasers.get(x).unwrap())).enumerate() {
        let mut last_active_switch_id = None;
        if column*button_width >= usable_width {
             column = 0.0;
             y_offset = next_y_offset;
        }
        let x_pos = x_offset + column*button_width;
        let current_edited_chaser_names = ui.current_edited_chaser_names.clone();
        if ui.edit_state {
            let ref mut current_chaser_name = current_edited_chaser_names.lock().expect("Failed to lock Arc!")[id];
            // let t: u8 = current_chaser_name;

            // let ref mut switch_name = switch_name.lock().expect("Failed to lock Arc!")[0];
            TextBox::new(current_chaser_name)
                .font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .xy_relative_to(CHASER_TITLE, [x_pos, y_offset])
                // .xy([0.0, 0.0])
                .w_h(button_width, button_height)
                .frame(2.0)
                .frame_color(app_theme.bg_control.plain_contrast())
                .color(app_theme.bg_control.invert().plain_contrast())
                .react(|_: &mut String| {})
                // .react(|new_name: &mut String| {
                //     let old_name = ui.chasers[id].clone();
                //     ui.frontend_data.rename_chaser(old_name.clone(), new_name.clone());
                //     ui.chasers = ui.chasers.iter().map(|x| if *x == old_name {new_name.clone()} else {x.clone()}).collect();
                //     ui.save_chaser_config();
                //     ui.send_data();
                // })
                .enabled(true)
                .set(EDITOR_CHASER_TITLE + id, conrod_ui);

            if button_pressed {
                let old_name = ui.chasers[id].clone();
                ui.frontend_data.rename_chaser(old_name.clone(), current_chaser_name.clone());
                ui.chasers = ui.chasers.iter().map(|x| if *x == old_name {current_chaser_name.clone()} else {x.clone()}).collect();
                ui.save_chaser_config();
                ui.send_data();
            }
        }
        else {
            Text::new(name)
                .xy_relative_to(CHASER_TITLE, [x_pos, y_offset])
                .font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .color(app_theme.bg_control.plain_contrast())
                .set(CONTROL_CHASER_TITLE + id, conrod_ui);
        }

        for (switch_id_in_chaser, (switch_id, switch)) in chaser.switches.iter().map(|&switch_id| (switch_id, &ui.frontend_data.switches[switch_id])).enumerate() {
            let y_pos = y_offset - 50.0 * app_theme.ui_scale - switch_id_in_chaser as f64*button_height;
            let current_edited_switch = ui.current_edited_switch_id.clone();

            let label = match switch.get_keybinding_as_text() {
                Some(keybinding) => switch.name.clone() + ": " + &keybinding,
                None => switch.name.clone()
            };
            Button::new()
                .w_h(button_width, button_height)
                .xy_relative_to(CHASER_TITLE, [x_pos, y_pos])
                .and(|b| {
                    if switch.dimmer_value != 0.0 {
                        last_active_switch_id = Some(switch_id_in_chaser);
                        b.color(app_theme.switch_on_color)
                    } else {
                        b.color(app_theme.switch_off_color)
                    }
                })
                .frame(1.0)
                .label(&label)
                .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .react(|| {
                    if ui.edit_state {
                        current_edited_switch.lock().expect("Failed to lock Arc!")[0] = Some(switch_id);
                        ui.current_edited_switch_name.lock().expect("Failed to lock Arc!")[0] = switch.name.clone();
                        if ui.control_state {
                            // ui.waiting_for_keybinding = true; //TODO make this working
                        }
                    }
                    else {
                        let new_value = if switch.dimmer_value == 0.0 {255} else {0};
                        tx.send(get_switch_update(ui.shift_state, switch_id as u16, new_value)).unwrap();
                    }
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
        let mut y_pos = y_offset - 50.0 * app_theme.ui_scale - (chaser.switches.len() as f64 - 0.25)*button_height;
        if !ui.edit_state {
            {
                let tx = tx.clone();
                //let x_pos = (id as f64 - 5f64/6f64) * button_width;
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos - button_width/3.0, y_pos])
                    .color(app_theme.chaser_control_color)
                    .frame(1.0)
                    .label(&"<<".to_string())
                    .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                    .react(|| {
                        let next_switch_id = {
                            match last_active_switch_id {
                                Some(last_active_switch_id) => {
                                    if last_active_switch_id == 0 {chaser.switches.len() - 1} else {last_active_switch_id - 1}
                                },
                                None => 0
                            }
                        };
                        tx.send(get_switch_update(!ui.shift_state, chaser.switches[next_switch_id] as u16, 255)).unwrap();
                    })
                    .set(current_button_id, conrod_ui);
                    current_button_id = current_button_id + 1;
            }
            {
                let tx = tx.clone();
                //let x_pos = (id as f64 - 0.5) * button_width;
                let (label, r) = {
                    if chaser.current_thread {
                        ("||".to_string(), 0.1)
                    }
                    else {
                        (">".to_string(), 0.9)
                    }
                };
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos, y_pos])
                    .color(app_theme.chaser_control_color)
                    .frame(1.0)
                    .label(&label)
                    .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                    .react(|| {
                        let next_switch_id = {
                            match last_active_switch_id {
                                Some(last_active_switch_id) => {
                                    if last_active_switch_id == 0 {chaser.switches.len() - 1} else {last_active_switch_id - 1}
                                },
                                None => 0
                            }
                        };
                        if chaser.current_thread {
                            tx.send(get_start_chaser(!ui.shift_state, chaser.switches[next_switch_id] as u16, 0)).unwrap();
                        }
                        else {
                            tx.send(get_start_chaser(!ui.shift_state, chaser.switches[next_switch_id] as u16, 255)).unwrap();
                        }
                    })
                    .set(current_button_id, conrod_ui);
                    current_button_id = current_button_id + 1;
            }
            {
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos + button_width/3.0, y_pos])
                    .color(app_theme.chaser_control_color)
                    .frame(1.0)
                    .label(&">>".to_string())
                    .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                    .react(|| {
                        let next_switch_id = {
                            match last_active_switch_id {
                                Some(last_active_switch_id) => {
                                    if last_active_switch_id + 1 == chaser.switches.len() {0} else {last_active_switch_id + 1}
                                },
                                None => 0
                            }
                        };
                        tx.send(get_switch_update(!ui.shift_state, chaser.switches[next_switch_id] as u16, 255)).unwrap();
                    })
                    .set(current_button_id, conrod_ui);
                    current_button_id = current_button_id + 1;
            }
        }
        else {
            let mut test = false; //TODO Fix this fake
            Button::new()
                .w_h(button_width, button_height/2.0)
                .xy_relative_to(CHASER_TITLE, [x_pos, y_pos])
                .color(app_theme.add_button_color)
                .frame(1.0)
                .label(&"Add".to_string())
                .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .react(|| {
                    let switch_id = ui.frontend_data.add_switch(JsonSwitch::new("Untitled".to_string(), name.clone()));
                    ui.current_edited_switch_id.lock().expect("Failed to lock Arc!")[0] = Some(switch_id);
                    ui.current_edited_switch_name.lock().expect("Failed to lock Arc!")[0] = "Untitled".to_string();

                    ui.send_data();
                    test = true;
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;

            Button::new()
                    .w_h(button_width, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos, y_pos - button_height/2.0])
                    .color(app_theme.remove_button_color)
                    .frame(1.0)
                    .label(&"Delete".to_string())
                    .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                    .react(|| {
                        ui.frontend_data.delete_chaser(name.clone());
                        ui.chasers.retain(|x| x != name);
                        ui.current_edited_chaser_names = Arc::new(Mutex::new(ui.chasers.clone()));
                        ui.save_chaser_config();
                        ui.send_data();
                        test = true;
                    })
                    .set(current_button_id, conrod_ui);
                    current_button_id = current_button_id + 1;

            y_pos = y_pos - button_height;
            if test
            {
                return;
            }
        }
        if y_pos - button_height < next_y_offset {
            next_y_offset = y_pos - button_height;
        }
        column += 1.0;
    }
}

fn draw_editor(mut conrod_ui: &mut UiCell, ui: &mut UI, app_theme: Theme, usable_width: f64, button_pressed: bool) {
    Text::new("Editor")
        .top_left_of(EDITOR_COLUMN)
        .font_size((22.0 * app_theme.ui_scale) as u32)
        .color(app_theme.bg_editor.plain_contrast())
        .set(EDITOR_TITLE, conrod_ui);


    let current_edited_switch = {
        ui.current_edited_switch_id.lock().expect("Failed to lock Arc!")[0].clone()
    };

    let switch_name = ui.current_edited_switch_name.clone();

    match current_edited_switch {
        Some(switch_id) => {

            let time = ui.frontend_data.switches[switch_id].before_chaser;
            let item_width = usable_width * app_theme.ui_scale;
            let item_height = usable_width/8.0 * app_theme.ui_scale;//40.0
            let item_x_offset = 20.0 * app_theme.ui_scale;
            let line = "-----------------------------------------";
            let ref mut switch_name = switch_name.lock().expect("Failed to lock Arc!")[0];


            TextBox::new(switch_name)
                .font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .down(20.0 * app_theme.ui_scale)
                .align_left_of(EDITOR_TITLE)
                .w_h(item_width, item_height)
                .frame(2.0)
                .frame_color(app_theme.bg_editor.plain_contrast())
                .color(app_theme.bg_editor.invert().plain_contrast())
                .react(|_: &mut String| {})
                // .react(|new_name: &mut String| {
                //     ui.frontend_data.switches[switch_id].name = new_name.clone();
                //     ui.send_data();
                // })
                .enabled(true)
                .set(EDITOR_CONTENT, conrod_ui);
            if button_pressed {
                ui.frontend_data.switches[switch_id].name = switch_name.clone();
                ui.send_data();
            }

            NumberDialer::new(time as f32, 0.0, 99999.0, 0)
                .w_h(item_width, item_height)
                .down(20.0 * app_theme.ui_scale)
                .align_left_of(EDITOR_TITLE)
                .color(app_theme.number_dialer_color)
                .frame(2.0)
                .label(&"Chaser time (ms)".to_string())
                .label_color(app_theme.font_color)
                .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .react(|new_time: f32| {
                    ui.frontend_data.switches[switch_id].before_chaser = new_time as FadeTime;
                    ui.send_data();
                })
                .set(EDITOR_TIME_NUMBER_DIALER, conrod_ui);

            let label = if ui.waiting_for_keybinding {
                "Keybinding: ?".to_string()
            }
            else {
                match ui.frontend_data.switches[switch_id].get_keybinding_as_text() {
                    Some(keybinding) => "Keybinding: ".to_string() + &keybinding,
                    None => "No keybinding".to_string()
                }
            };

            let mut editor_switch_slider_count = 0;
            let mut editor_switch_number_dialer_count = 0;
            let mut editor_switch_button_count = 0;
            let mut editor_switch_text_count = 0;
            let mut editor_switch_drop_downs_count = 0;


            Button::new()
                .w_h(item_width, item_height)
                .down(20.0 * app_theme.ui_scale)
                .align_left_of(EDITOR_TITLE)
                .color(app_theme.chaser_control_color)
                .frame(1.0)
                .label(&label)
                .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .react(|| {
                    ui.waiting_for_keybinding = true;
                })
                .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                editor_switch_button_count += 1;

            Text::new(line)
            .down(20.0 * app_theme.ui_scale)
            .align_left_of(EDITOR_TITLE)
                .font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .color(app_theme.bg_editor.plain_contrast())
                .set(EDITOR_SWITCH_TEXT + editor_switch_text_count, conrod_ui);
            editor_switch_text_count += 1;

            let cloned_ui = ui.clone();

            let mut data: Vec<String> = cloned_ui.frontend_data.switches[switch_id].channel_groups.keys().map(|x| x.clone()).collect();
            data.sort();

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
                    .down(20.0 * app_theme.ui_scale)
                    .align_left_of(EDITOR_TITLE)
                    .color(app_theme.drop_down_list_color)
                    .frame(2.0)
                    .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                    .label_color(app_theme.font_color)
                    .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                    .react(|_: &mut Option<usize>, new_idx, _: &str| {
                        if ui.frontend_data.change_channel_group(switch_id, id_string.clone(), dropdown_background_list_fixture[new_idx], dropdown_background_list_channel_groups[new_idx]) {
                            ui.current_edited_channel_group_id = new_idx as i64;
                            ui.send_data();
                        }
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
                    .right(0.0)
                    .color(app_theme.chaser_control_color)
                    .frame(1.0)
                    .label(&label)
                    .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                    .react(|| {
                        if dropdown_index as i64 == ui.current_edited_channel_group_id {
                            ui.current_edited_channel_group_id = -1;
                        }
                        else {
                            ui.current_edited_channel_group_id = dropdown_index as i64;
                            let mut current_edited_curve_strings_locked = ui.current_edited_curve_strings.lock().expect("Failed to lock Arc!");
                            current_edited_curve_strings_locked[0] = data.curve_in.get_string();
                            current_edited_curve_strings_locked[1] = data.curve_out.get_string();
                        };
                    })
                    .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                editor_switch_button_count += 1;


                if dropdown_index as i64 == ui.current_edited_channel_group_id {
                    for (index, &value) in data.values.iter().enumerate() {
                        let label = {
                            let mut text = "Value: ".to_string();
                            text.push_str(&value.to_string());
                            text
                        };
                        Slider::new(value as f32, 0.0, 255.0)
                            .w_h(item_width - item_x_offset, item_height)
                            .down(20.0 * app_theme.ui_scale)
                            .align_right_of(EDITOR_CONTENT)
                            .color(app_theme.slider_color)
                            .frame(2.0)
                            .label(&label)
                            .label_color(app_theme.font_color)
                            .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                            .react(|new_value: f32| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().values[index] = new_value as u8;
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_NUMBER_DIALER + editor_switch_number_dialer_count, conrod_ui);
                        editor_switch_number_dialer_count += 1;
                    }

                    let mut fade_curve_list = vec!("Linear".to_string(), "Squared".to_string(), "Square root".to_string(), "Custom".to_string());

                    {
                        let data = ui.frontend_data.switches[switch_id].channel_groups.get(id_string).unwrap().clone();

                        let fade_curve_id = data.curve_in.get_id();
                        DropDownList::new(&mut fade_curve_list, &mut Some(fade_curve_id))
                            .w_h(item_width - item_x_offset, item_height)
                            .down(20.0 * app_theme.ui_scale)
                            .align_right_of(EDITOR_CONTENT)
                            .color(app_theme.drop_down_list_color)
                            .frame(2.0)
                            .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                            .label_color(app_theme.font_color)
                            .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                            .react(|_: &mut Option<usize>, new_idx, _: &str| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::get_by_id(new_idx, "x".to_string());
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                        editor_switch_drop_downs_count += 1;

                        if fade_curve_id == 3 {
                            let ref mut curve_string = {ui.current_edited_curve_strings.lock().expect("Failed to lock Arc!")[0].clone()};

                            TextBox::new(curve_string)
                                .w_h(item_width - item_x_offset, item_height)
                                .down(20.0 * app_theme.ui_scale)
                                .align_right_of(EDITOR_CONTENT)
                                .font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                                .frame(2.0)
                                .frame_color(app_theme.bg_editor.plain_contrast())
                                .color(app_theme.bg_editor.invert().plain_contrast())
                                .react(|_: &mut String| {})
                                // .react(|new_name: &mut String| {
                                //     ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::Custom(new_name.clone());
                                //     ui.send_data();
                                // })
                                .enabled(true)
                                .set(EDITOR_CURVE_STRING1, conrod_ui);

                            if button_pressed {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::Custom(curve_string.clone());
                                ui.send_data();
                            }
                        }

                        NumberDialer::new(data.time_in as f32, 0.0, 99999.0, 0)
                            .w_h(item_width - item_x_offset, item_height)
                            .down(20.0 * app_theme.ui_scale)
                            .align_right_of(EDITOR_CONTENT)
                            .color(app_theme.number_dialer_color)
                            .frame(2.0)
                            .label(&"Time in".to_string())
                            .label_color(app_theme.font_color)
                            .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                            .react(|new_value: f32| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().time_in = new_value as FadeTime;
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_NUMBER_DIALER + editor_switch_number_dialer_count, conrod_ui);
                        editor_switch_number_dialer_count += 1;

                        let fade_curve_id = data.curve_out.get_id();
                        DropDownList::new(&mut fade_curve_list, &mut Some(fade_curve_id))
                            .w_h(item_width - item_x_offset, item_height)
                            .down(20.0 * app_theme.ui_scale)
                            .align_right_of(EDITOR_CONTENT)
                            .color(app_theme.drop_down_list_color)
                            .frame(2.0)
                            .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                            .label_color(app_theme.font_color)
                            .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                            .react(|_: &mut Option<usize>, new_idx, _: &str| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::get_by_id(new_idx, "x".to_string());
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                        editor_switch_drop_downs_count += 1;

                        if fade_curve_id == 3 {
                            let ref mut curve_string = {ui.current_edited_curve_strings.lock().expect("Failed to lock Arc!")[1].clone()};

                            TextBox::new(curve_string)
                                .w_h(item_width - item_x_offset, item_height)
                                .down(20.0 * app_theme.ui_scale)
                                .align_right_of(EDITOR_CONTENT)
                                .frame(2.0)
                                .frame_color(app_theme.bg_editor.plain_contrast())
                                .color(app_theme.bg_editor.invert().plain_contrast())
                                .react(|_: &mut String| {})
                                // .react(|new_name: &mut String| {
                                //     ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::Custom(new_name.clone());
                                //     ui.send_data();
                                // })
                                .enabled(true)
                                .set(EDITOR_CURVE_STRING2, conrod_ui);
                            if button_pressed {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::Custom(curve_string.clone());
                                ui.send_data();
                            }
                        }

                        NumberDialer::new(data.time_out as f32, 0.0, 99999.0, 0)
                            .w_h(item_width - item_x_offset, item_height)
                            .down(20.0 * app_theme.ui_scale)
                            .align_right_of(EDITOR_CONTENT)
                            .color(app_theme.number_dialer_color)
                            .frame(2.0)
                            .label(&"Time out".to_string())
                            .label_color(app_theme.font_color)
                            .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                            .react(|new_value: f32| {
                                ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().time_out = new_value as FadeTime;
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_NUMBER_DIALER + editor_switch_number_dialer_count, conrod_ui);
                        editor_switch_number_dialer_count += 1;
                    }

                    Button::new()
                        .w_h(item_width - item_x_offset, item_height)
                        .down(20.0 * app_theme.ui_scale)
                        .align_right_of(EDITOR_CONTENT)
                        .color(app_theme.remove_button_color)
                        .frame(1.0)
                        .label("Delete")
                        .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                        .react(|| {
                            ui.frontend_data.remove_channel_group(switch_id, id_string.clone());
                            ui.send_data();
                        })
                        .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                        editor_switch_button_count += 1;
                }
            }
            Button::new()
                .w_h(item_width, item_height)
                .down(20.0 * app_theme.ui_scale)
                .align_left_of(EDITOR_TITLE)
                .color(app_theme.add_button_color)
                .frame(1.0)
                .label("Add")
                .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .react(|| {
                    ui.frontend_data.add_channel_group(switch_id);
                    ui.send_data();
                })
                .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
            editor_switch_button_count += 1;

            Button::new()
                .w_h(item_width, item_height)
                .down(20.0 * app_theme.ui_scale)
                .align_left_of(EDITOR_TITLE)
                .color(app_theme.remove_button_color)
                .frame(1.0)
                .label("Delete")
                .label_font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .react(|| {
                    ui.frontend_data.remove_switch_with_id(switch_id);
                    ui.current_edited_switch_id.lock().expect("Failed to lock Arc!")[0] = None;
                    ui.send_data();
                })
                .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
        }
        None => {
            Text::new("No switch selected")
                .down(20.0 * app_theme.ui_scale)
                .align_left_of(EDITOR_TITLE)
                .font_size((app_theme.base_font_size * app_theme.ui_scale) as u32)
                .color(app_theme.font_color.plain_contrast())
                .set(EDITOR_INFO, conrod_ui);
        }
    }
}

fn main() {
    init_logger();
    info!("BitDMX frontend v{}-{}", VERSION, GIT_HASH);

    let ui = UI::new();
    SplashWindow::new(ui.clone()).join().unwrap();
    if {ui.lock().expect("Failed to lock Arc!").watchdog.is_alive()} { create_output_window(ui.clone()); }
}
