use gpui::{App, Hsla, WindowAppearance};

pub struct ThemeValues {
    pub white: Hsla,
    pub transparent: Hsla,

    pub app_bg: Hsla,
    pub dropzone_active: Hsla,
    pub file_bg_sequence_1: Hsla,
    pub status_bar_bg: Hsla,

    pub base_text_color: Hsla,
    pub secondary_text_color: Hsla,

    pub progress_color: Hsla,

    pub warning_text_color: Hsla,
    pub danger_text_color: Hsla,
    pub success_text_color: Hsla,
}

pub struct Theme {
    pub light: ThemeValues,
    pub dark: ThemeValues,
    pub default: ThemeValues,
}

const DARK_THEME: ThemeValues = ThemeValues {
    white: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 1.0,
    },
    transparent: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    },

    app_bg: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.7,
    },
    dropzone_active: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.1,
    },
    status_bar_bg: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 0.075,
    },
    file_bg_sequence_1: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 0.05,
    },

    base_text_color: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.95,
        a: 1.0,
    },
    secondary_text_color: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.596,
        a: 1.0,
    },

    progress_color: Hsla {
        h: 0.6,
        s: 0.74,
        l: 0.51,
        a: 1.0,
    },

    warning_text_color: Hsla {
        h: 0.152777778,
        s: 0.69,
        l: 0.48,
        a: 1.0,
    },
    danger_text_color: Hsla {
        h: 0.0,
        s: 0.65,
        l: 0.52,
        a: 1.0,
    },
    success_text_color: Hsla {
        h: 0.363888889,
        s: 0.69,
        l: 0.48,
        a: 1.0,
    },
};

const LIGHT_THEME: ThemeValues = ThemeValues {
    white: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 1.0,
    },
    transparent: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 0.0,
    },

    app_bg: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 0.85,
    },
    dropzone_active: Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 0.1,
    },
    status_bar_bg: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.051,
    },
    file_bg_sequence_1: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.051,
    },

    base_text_color: Hsla {
        h: 0.0,
        s: 0.02,
        l: 0.10,
        a: 1.0,
    },
    secondary_text_color: Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.502,
        a: 1.0,
    },

    progress_color: Hsla {
        h: 0.6,
        s: 0.82,
        l: 0.45,
        a: 1.0,
    },

    danger_text_color: Hsla {
        h: 0.0,
        s: 0.62,
        l: 0.47,
        a: 1.0,
    },
    warning_text_color: Hsla {
        h: 0.152777778,
        s: 0.83,
        l: 0.35,
        a: 1.0,
    },
    success_text_color: Hsla {
        h: 0.363888889,
        s: 0.70,
        l: 0.42,
        a: 1.0,
    },
};

const DEFAULT_THEME: ThemeValues = LIGHT_THEME;

pub const APP_THEME: Theme = Theme {
    light: LIGHT_THEME,
    dark: DARK_THEME,
    default: DEFAULT_THEME,
};

pub fn get_theme(cx: &App) -> ThemeValues {
    match cx.window_appearance() {
        WindowAppearance::Dark => APP_THEME.dark,
        WindowAppearance::Light => APP_THEME.light,
        _ => APP_THEME.default,
    }
}

pub fn get_logo(cx: &App) -> String {
    match cx.window_appearance() {
        WindowAppearance::Dark => "logo-1.png".to_owned(),
        WindowAppearance::Light => "logo-2.png".to_owned(),
        _ => "logo-2.png".to_owned(),
    }
}
