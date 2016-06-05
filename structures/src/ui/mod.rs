//! Helpers, wrappers and structs to make programming the UI just that little bit easier
pub mod colors;
pub mod window;
pub mod frontend_data;
pub mod ui;
pub mod theme;
pub mod frontend_config;
pub mod error_window;

pub use ui::ui::UI;
pub use ui::theme::Theme;
pub use ui::frontend_data::FrontendData;
