use dark_light;
use iced::{color, Color};

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
/// Color scheme
pub enum Theme {
    #[default]
    /// `Dark` or `Light`, according to `dark_light`
    Auto,
    /// `Dark`-ish and purple
    Lupin,
    /// white on black
    Dark,
    /// black on white
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    #[allow(dead_code)]
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub const ALL: [Self; 4] = [Self::Auto, Self::Lupin, Self::Dark, Self::Light];

    pub fn palette(self) -> ColorPalette {
        const DARK: ColorPalette = ColorPalette {
            base: BaseColors {
                background: color!(0x11_11_11),
                foreground: color!(0x1C_1C_1C),
            },
            normal: NormalColors {
                primary: color!(0x5E_42_66),
                secondary: color!(0x38_6E_50),
                surface: color!(0x82_82_82),
                error: color!(0x99_2B_2B),
            },
            bright: BrightColors {
                primary: color!(0xBA_84_FC),
                secondary: color!(0x49_EB_7A),
                surface: color!(0xE0_E0_E0),
                error: color!(0xC1_30_47),
            },
        };
        const LIGHT: ColorPalette = ColorPalette {
            base: BaseColors {
                background: color!(0xEE_EE_EE),
                foreground: color!(0xE0_E0_E0),
            },
            normal: NormalColors {
                primary: color!(0x81_81_81),
                secondary: color!(0xF9_D6_59),
                surface: color!(0x81_81_81),
                error: color!(0x99_2B_2B),
            },
            bright: BrightColors {
                primary: color!(0x67_3A_B7),
                secondary: color!(0x37_97_A4),
                surface: color!(0x00_00_00),
                error: color!(0xC1_30_47),
            },
        };
        const LUPIN: ColorPalette = ColorPalette {
            base: BaseColors {
                background: color!(0x28_2A_36),
                foreground: color!(0x35_37_46),
            },
            normal: NormalColors {
                primary: color!(0x58_40_6F),
                secondary: color!(0x38_6E_50),
                surface: color!(0xA2_A4_A3),
                error: color!(0xA1_30_34),
            },
            bright: BrightColors {
                primary: color!(0xBD_94_F9),
                secondary: color!(0x49_EB_7A),
                surface: color!(0xF4_F8_F3),
                error: color!(0xE6_3E_6D),
            },
        };
        match self {
            Self::Dark => DARK,
            Self::Light => LIGHT,
            Self::Lupin => LUPIN,
            #[allow(clippy::match_same_arms)]
            Self::Auto => match dark_light::detect() {
                dark_light::Mode::Light => LIGHT,
                dark_light::Mode::Dark => DARK,
                dark_light::Mode::Default => DARK,
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "Dark",
                Self::Light => "Light",
                Self::Lupin => "Lupin",
                Self::Auto => "Auto (follow system theme)",
            }
        )
    }
}
