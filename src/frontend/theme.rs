use conrod::color;
use colors::*;

#[derive(Debug, Clone)]
pub struct Theme { //TODO add colors
     pub ui_scale: f64,
     pub base_font_size: f64,
     pub keybindings_font_size: f64,
     pub ui_padding: f64,
     pub bg_header: color::Color,
     pub bg_control: color::Color,
     pub bg_editor: color::Color,
     pub add_button_color: color::Color,
     pub remove_button_color: color::Color,
     pub switch_on_color: color::Color,
     pub switch_off_color: color::Color,
     pub chaser_control_color: color::Color,
     pub font_color: color::Color,
     pub slider_color: color::Color,
     pub number_dialer_color: color::Color,
     pub drop_down_list_color: color::Color
}

impl Theme {
    pub fn default() -> Theme {
        Theme {
            ui_scale: 0.8,
            base_font_size: 20.0,
            keybindings_font_size: 15.0,
            ui_padding: 10.0,
            bg_header: midnight_blue(),
            bg_control: concrete(),
            bg_editor: concrete(),
            add_button_color: emerald(),
            remove_button_color: alizarin(),
            switch_on_color: nephritis(),
            switch_off_color: pomegranate(),
            chaser_control_color: pumpkin(),
            font_color: color::rgb(0.0, 0.0, 0.0),
            slider_color: emerald(),
            number_dialer_color: turquoise(),
            drop_down_list_color: wisteria()
        }
    }
}
