use std::sync::{Arc, Mutex};
use conrod::{
    Colorable,
    Frameable,
    Positionable,
    Text,
    Widget,
    Button,
    Labelable,
    Sizeable,
};

use ui::UI;
use window::*;
use theme::Theme;

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

pub fn draw_header(mut conrod_ui: &mut UiCell, ui: &mut UI, application_theme: Theme) {
    Text::new("Moonshadow 2016!")
        .top_left_of(HEADER)
        .font_size((32.0 * application_theme.ui_scale) as u32)
        .color(application_theme.bg_color.plain_contrast())
        .set(TITLE, conrod_ui);

    let connected = ui.watchdog.is_alive();
    let label = if connected { "Connected".to_string() } else { "Disconnected".to_string() };
    Button::new()
        .w_h(105.0 * application_theme.ui_scale, 35.0 * application_theme.ui_scale)
        .down_from(TITLE, 7.0 * application_theme.ui_scale)
        .frame(1.0)
        .label(&label)
        .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
        .and(|b| {
            if connected {
                b.rgb(0.1, 0.9, 0.1)
            } else {
                b.rgb(0.9, 0.1, 0.1)
            }
        })
        .react(|| {})
        .set(CONNECTED_BUTTON, conrod_ui);

    Button::new()
        .w_h(105.0 * application_theme.ui_scale, 35.0 * application_theme.ui_scale)
        .right_from(CONNECTED_BUTTON, 5.0 * application_theme.ui_scale)
        .frame(1.0)
        .label(&"Edit Mode".to_string())
        .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
        .and(|b| {
            if ui.edit_state {
                b.rgb(0.1, 0.9, 0.1)
            } else {
                b.rgb(0.9, 0.1, 0.1)
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
            .w_h(105.0 * application_theme.ui_scale, 35.0 * application_theme.ui_scale)
            .right_from(EDITOR_BUTTON, 5.0 * application_theme.ui_scale)
            .frame(1.0)
            .label(&"Add Chaser".to_string())
            .label_font_size((application_theme.base_font_size * application_theme.ui_scale) as u32)
            .and(|b| {
                if ui.edit_state {
                    b.rgb(0.1, 0.9, 0.1)
                } else {
                    b.rgb(0.9, 0.1, 0.1)
                }
            })
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
