use gpui::Rgba;

pub struct ThemeValues {
    pub base_color: Rgba,
    pub base_text_color: Rgba,
}

pub struct Theme {
    pub light: ThemeValues,
    pub dark: ThemeValues,
    pub default: ThemeValues,
}

const DARK_THEME: ThemeValues = ThemeValues {
    base_color: Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.15,
    },
    base_text_color: Rgba {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
};

const LIGHT_THEME: ThemeValues = ThemeValues {
    base_color: Rgba {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.15,
    },
    base_text_color: Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
};

const DEFAULT_THEME: ThemeValues = LIGHT_THEME;

pub const APP_THEME: Theme = Theme {
    light: LIGHT_THEME,
    dark: DARK_THEME,
    default: DEFAULT_THEME,
};
