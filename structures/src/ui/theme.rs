//! All those nifty themes for BitDMX
use conrod::color;
use ui::colors::FlatColor;

/// A collection of colors and floats to define the look of the UI
#[derive(Debug, Clone)]
pub struct Theme {
    /// Scaling factor for the UI
    pub ui_scale: f64,
    /// Base font size which is multiplied with the `ui_scale`
    pub base_font_size: f64,
    /// Font size for the keybindings
    pub keybindings_font_size: f64,
    /// Padding used across the UI
    pub ui_padding: f64,
    /// Background color for header
    pub bg_header: color::Color,
    /// I've got no idea, what this is doing
    pub bg_control: color::Color,
    /// Background color for the editor
    pub bg_editor: color::Color,
    /// Color for the add button
    pub add_button_color: color::Color,
    /// Color for the remove button
    pub remove_button_color: color::Color,
    /// Color for an active switch
    pub switch_on_color: color::Color,
    /// Color for an inactive switch
    pub switch_off_color: color::Color,
    /// Color for a selected switch in editor mode
    pub selected_switch_color: color::Color,
    /// No idea what this is doin'
    pub chaser_control_color: color::Color,
    /// Color used in font rendering
    pub font_color: color::Color,
    /// Color for sliders
    pub slider_color: color::Color,
    /// Background color used for number dialers
    pub number_dialer_color: color::Color,
    /// Background color for dropdown lists
    pub drop_down_list_color: color::Color
}

impl Theme {
    /// Create a new instance of the default theme
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

/// Get a list of all available themes
pub fn get_themes() -> Vec<(String, Theme)> {
    vec!(("Default".to_string(), Theme::default()))
}
