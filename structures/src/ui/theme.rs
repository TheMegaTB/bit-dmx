use conrod::color;
use ui::colors::FlatColor;

#[derive(Debug, Clone)]
pub struct Theme {
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
     pub selected_switch_color: color::Color,
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
            bg_header: FlatColor::midnight_blue(),
            bg_control: FlatColor::concrete(),
            bg_editor: FlatColor::concrete(),
            add_button_color: FlatColor::emerald(),
            remove_button_color: FlatColor::alizarin(),
            switch_on_color: FlatColor::nephritis(),
            switch_off_color: FlatColor::pomegranate(),
            selected_switch_color: FlatColor::belize_hole(),
            chaser_control_color: FlatColor::pumpkin(),
            font_color: color::rgb(0.0, 0.0, 0.0),
            slider_color: FlatColor::emerald(),
            number_dialer_color: FlatColor::turquoise(),
            drop_down_list_color: FlatColor::wisteria()
        }
    }
}

pub fn get_themes() -> Vec<(String, Theme)> {
    vec!(("Default".to_string(), Theme::default()))
}
