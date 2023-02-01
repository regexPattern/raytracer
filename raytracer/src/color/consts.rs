use super::Color;

// TODO: Eventually I plan to build a tool that converts a given color (e.g. in hex format) to it's
// green, red and blue component percentages. For the time being `https://encycolorpedia.com` is a
// good resource to get these values.

/// HTML color: #FFFFFF
pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

/// HTML color: #000000
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

/// HTML color: #FF0000
pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};

/// HTML color: #00FF00
pub const GREEN: Color = Color {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};

/// HTML color: #0000FF
pub const BLUE: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};

/// HTML color: #87CEFA
pub const LIGHT_SKY_BLUE: Color = Color {
    red: 0.5294,
    green: 0.8078,
    blue: 0.9804,
};

/// HTML color: #9B7653
pub const DIRT: Color = Color {
    red: 0.6078,
    green: 0.4627,
    blue: 0.3255,
};
