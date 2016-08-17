
use conrod::{
    Colorable,
    Frameable,
    Positionable,
    Text,
    Widget,
    Button,
    Labelable,
    Sizeable,
    TextBox
};

use ui::UI;
use window::*;
use theme::*;
use frontend_helpers::*;
use structures::JsonSwitch;

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


pub fn draw_chasers(mut conrod_ui: &mut UiCell, ui: &mut UI, application_theme: Theme, usable_width: f64) {
    let tx = ui.tx.clone();

    let button_width = 200.0 * application_theme.ui_scale;
    let button_height = 50.0 * application_theme.ui_scale;
    let mut current_button_id = BUTTON;

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
        .font_size((22.0 * application_theme.ui_scale) as u32)
        .color(application_theme.bg_color.plain_contrast())
        .set(CHASER_TITLE, conrod_ui);


    let mut next_y_offset = 0f64;
    let x_offset = button_width/2.0 - TEXT_BLOCK_WIDTH/2.0;
    let mut column = 0.0;
    let mut y_offset = -50.0 * application_theme.ui_scale;

    let cloned_ui = ui.clone();


    for (id, (name, chaser)) in chasers.iter().map(|x| (x, cloned_ui.frontend_data.chasers.get(x).unwrap())).enumerate() {
        let mut last_active_switch_id = None;
        if (column + 1.0)*button_width >= usable_width {
             column = 0.0;
             y_offset = next_y_offset;
        }
        let x_pos = x_offset + column*button_width;
        let current_editor_chaser_names = ui.current_editor_chaser_names.clone();
        if ui.editor_state {
            // let tmp_name = {current_editor_chaser_names.lock().expect("Failed to lock Arc!")[id].clone()};
            let ref mut current_chaser_name = lock!(current_editor_chaser_names)[id];

            // let ref mut switch_name = switch_name.lock().expect("Failed to lock Arc!")[0];
            TextBox::new(current_chaser_name)
                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .xy_relative_to(CHASER_TITLE, [x_pos, y_offset])
                // .xy([0.0, 0.0])
                .w_h(button_width, button_height)
                .frame(2.0)
                .frame_color(application_theme.bg_color.invert().plain_contrast())
                .color(application_theme.bg_color.plain_contrast())
                .react(|new_name: &mut String| {
                    let old_name = ui.chasers[id].clone();
                    ui.frontend_data.rename_chaser(old_name.clone(), new_name.clone());
                    ui.chasers = ui.chasers.iter().map(|x| if *x == old_name {new_name.clone()} else {x.clone()}).collect();
                    ui.save_chaser_config();
                    ui.send_data();
                })
                .enabled(true)
                .set(EDITOR_CHASER_TITLE + id, conrod_ui);
        }
        else {
            Text::new(name)
                .xy_relative_to(CHASER_TITLE, [x_pos, y_offset])
                .font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .color(application_theme.bg_color.plain_contrast())
                .set(CONTROL_CHASER_TITLE + id, conrod_ui);
        }

        for (switch_id_in_chaser, (switch_id, switch)) in chaser.switches.iter().map(|&switch_id| (switch_id, &ui.frontend_data.switches[switch_id])).enumerate() {
            let y_pos = y_offset - 50.0 * application_theme.ui_scale - switch_id_in_chaser as f64*button_height;
            let current_editor_switch = ui.current_editor_switch_id.clone();

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
                        b.rgb(0.1, 0.9, 0.1)
                    } else {
                        b.rgb(0.9, 0.1, 0.1)
                    }
                })
                .frame(1.0)
                .label(&label)
                .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .react(|| {
                    if ui.editor_state {
                        lock!(current_editor_switch)[0] = Some(switch_id);
                        lock!(ui.current_editor_switch_name)[0] = switch.name.clone();
                    }
                    else {
                        let new_value = if switch.dimmer_value == 0.0 {255} else {0};
                        tx.send(get_switch_update(ui.shift_state, switch_id as u16, new_value)).unwrap();
                    }
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
        let mut y_pos = y_offset - 50.0 * application_theme.ui_scale - (chaser.switches.len() as f64 - 0.25)*button_height;
        if !ui.editor_state {
            {
                let tx = tx.clone();
                //let x_pos = (id as f64 - 5f64/6f64) * button_width;
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos - button_width/3.0, y_pos])
                    .rgb(0.9, 0.9, 0.1)
                    .frame(1.0)
                    .label(&"<<".to_string())
                    .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
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
                    .rgb(r, 0.9, 0.1)
                    .frame(1.0)
                    .label(&label)
                    .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
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
                //let x_pos = (id as f64 - 1f64/6f64) * button_width;
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos + button_width/3.0, y_pos])
                    .rgb(0.9, 0.9, 0.1)
                    .frame(1.0)
                    .label(&">>".to_string())
                    .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
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
                .rgb(0.9, 0.9, 0.1)
                .frame(1.0)
                .label(&"Add".to_string())
                .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                .react(|| {
                    let switch_id = ui.frontend_data.add_switch(JsonSwitch::new("Untitled".to_string(), name.clone()));
                    lock!(ui.current_editor_switch_id)[0] = Some(switch_id);
                    lock!(ui.current_editor_switch_name)[0] = "Untitled".to_string();

                    ui.send_data();
                    test = true;
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;

            Button::new()
                    .w_h(button_width, button_height/2.0)
                    .xy_relative_to(CHASER_TITLE, [x_pos, y_pos - button_height/2.0])
                    .rgb(0.9, 0.1, 0.1)
                    .frame(1.0)
                    .label(&"Delete".to_string())
                    .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
                    .react(|| {
                        ui.frontend_data.delete_chaser(name.clone());
                        ui.chasers.retain(|x| x != name);
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
