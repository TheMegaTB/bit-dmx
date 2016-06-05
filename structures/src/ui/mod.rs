//! Helpers, wrappers and structs to make programming the UI just that little bit easier

/// Module to save all the flat colors used in the uis.
pub mod colors;
/// Module for a window handler.
pub mod window;
/// Module for the frontend data.
pub mod frontend_data;
/// Module for the information about the frontend ui.
pub mod ui;
/// Module for the frontend themese.
pub mod theme;
/// Module for the configuration of the frontend.
pub mod frontend_config;
/// Module for a error (and in the future bug report) window
pub mod error_window;

pub use ui::ui::UI;
pub use ui::theme::Theme;
pub use ui::frontend_data::FrontendData;
