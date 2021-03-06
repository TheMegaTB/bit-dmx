//! Some nice predefined colors
use conrod::color::{rgb, Color};

/// FlatUI styled colors from https://flatuicolors.com/
///
/// # Examples
/// ```
/// # extern crate conrod;
/// # extern crate structures;
/// # fn main() {
/// # use conrod::color::{rgb, Color};
/// use structures::ui::colors::FlatColor;
///
/// let color = FlatColor::turquoise();
/// # assert_eq!(rgb(0.102, 0.737, 0.612), color);
/// # }
/// ```
pub struct FlatColor;

impl FlatColor {
    #[allow(dead_code)]
    /// Returns the color turquoise
    pub fn turquoise() -> Color {
        rgb(0.102, 0.737, 0.612)
    }
    #[allow(dead_code)]
    /// Returns the color green sea
    pub fn green_sea() -> Color {
        rgb(0.086, 0.627, 0.522)
    }
    #[allow(dead_code)]
    /// Returns the color turquoise
    pub fn emerald() -> Color {
        rgb(0.18, 0.8, 0.443)
    }
    #[allow(dead_code)]
    /// Returns the color nephritis
    pub fn nephritis() -> Color {
        rgb(0.153, 0.682, 0.376)
    }
    #[allow(dead_code)]
    /// Returns the color peter river
    pub fn peter_river() -> Color {
        rgb(0.204, 0.596, 0.859)
    }
    #[allow(dead_code)]
    /// Returns the color belize hole
    pub fn belize_hole() -> Color {
        rgb(0.161, 0.502, 0.725)
    }
    #[allow(dead_code)]
    /// Returns the color amethyst
    pub fn amethyst() -> Color {
        rgb(0.608, 0.349, 0.714)
    }
    #[allow(dead_code)]
    /// Returns the color wisteria
    pub fn wisteria() -> Color {
        rgb(0.557, 0.267, 0.678)
    }
    #[allow(dead_code)]
    /// Returns the color wet asphalt
    pub fn wet_asphalt() -> Color {
        rgb(0.204, 0.286, 0.369)
    }
    #[allow(dead_code)]
    /// Returns the color midnight blue
    pub fn midnight_blue() -> Color {
        rgb(0.173, 0.243, 0.314)
    }
    #[allow(dead_code)]
    /// Returns the color sun flower
    pub fn sun_flower() -> Color {
        rgb(0.945, 0.769, 0.059)
    }
    #[allow(dead_code)]
    /// Returns the color orange
    pub fn orange() -> Color {
        rgb(0.953, 0.612, 0.071)
    }
    #[allow(dead_code)]
    /// Returns the color carrot
    pub fn carrot() -> Color {
        rgb(0.902, 0.494, 0.133)
    }
    #[allow(dead_code)]
    /// Returns the color pumpkin
    pub fn pumpkin() -> Color {
        rgb(0.827, 0.329, 0.0)
    }
    #[allow(dead_code)]
    /// Returns the color alizarin
    pub fn alizarin() -> Color {
        rgb(0.906, 0.298, 0.235)
    }
    #[allow(dead_code)]
    /// Returns the color pomegranate
    pub fn pomegranate() -> Color {
        rgb(0.753, 0.224, 0.169)
    }
    #[allow(dead_code)]
    /// Returns the color clouds
    pub fn clouds() -> Color {
        rgb(0.925, 0.941, 0.945)
    }
    #[allow(dead_code)]
    /// Returns the color silver
    pub fn silver() -> Color {
        rgb(0.741, 0.765, 0.78)
    }
    #[allow(dead_code)]
    /// Returns the color concrete
    pub fn concrete() -> Color {
        rgb(0.584, 0.647, 0.651)
    }
    #[allow(dead_code)]
    /// Returns the color asbestos
    pub fn asbestos() -> Color {
        rgb(0.498, 0.549, 0.553)
    }
    #[allow(dead_code)]
    /// Returns the color ebony clay
    pub fn ebony_clay() -> Color {
        rgb(0.133, 0.191, 0.246)
    }
    #[allow(dead_code)]
    /// Returns the color pickled bluewood
    pub fn pickled_bluewood() -> Color {
        rgb(0.203, 0.285, 0.367)
    }
    #[allow(dead_code)]
    /// Returns the color gray
    pub fn gray() -> Color {
        rgb(0.242, 0.273, 0.316)
    }
}
