use crate::style::{ClassStyle, StyleBuilder};
use std::{
    borrow::Borrow,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    iter,
};

/// The 8 colors defined by the original specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorName {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
impl ColorName {
    fn from_code(code: usize) -> Option<Self> {
        use ColorName::*;
        [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White]
            .get(code % 10)
            .copied()
    }

    /// Get the 24-bit colour code.
    pub fn rgb(self, bright: bool) -> u32 {
        use ColorName::*;

        macro_rules! rgb {
            ($r:literal, $g:literal, $b:literal) => {
                (($r << 16) + ($g << 8) + $b) as u32
            };
        }

        if bright {
            match self {
                Black => rgb!(1, 1, 1),
                Red => rgb!(222, 56, 43),
                Green => rgb!(57, 181, 74),
                Yellow => rgb!(255, 199, 6),
                Blue => rgb!(0, 111, 184),
                Magenta => rgb!(118, 38, 113),
                Cyan => rgb!(44, 181, 233),
                White => rgb!(204, 204, 204),
            }
        } else {
            match self {
                Black => rgb!(128, 128, 128),
                Red => rgb!(255, 0, 0),
                Green => rgb!(0, 255, 0),
                Yellow => rgb!(255, 255, 0),
                Blue => rgb!(0, 0, 255),
                Magenta => rgb!(255, 0, 255),
                Cyan => rgb!(0, 255, 255),
                White => rgb!(255, 255, 255),
            }
        }
    }
}
impl Display for ColorName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ColorName::*;
        let name = match self {
            Black => "black",
            Red => "red",
            Green => "green",
            Yellow => "yellow",
            Blue => "blue",
            Magenta => "magenta",
            Cyan => "cyan",
            White => "white",
        };
        f.write_str(name)
    }
}

/// Select Graphic Rendition parameter.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Sgr {
    Reset,
    Bold,
    Dim,
    BoldOff,
    Italic,
    ItalicOff,
    Underline,
    UnderlineOff,
    Reverse,
    ReverseOff,
    ColorFgRgb(u32),
    ColorFgName(ColorName),
    ColorFgNameBright(ColorName),
    ResetColorFg,
    ColorBgRgb(u32),
    ColorBgName(ColorName),
    ColorBgNameBright(ColorName),
    ResetColorBg,
}
impl Sgr {
    fn from_color_code(code: usize, background: bool, bright: bool) -> Option<Self> {
        use Sgr::*;
        let color = ColorName::from_code(code)?;
        let sgr = match (background, bright) {
            (false, false) => ColorFgName(color),
            (false, true) => ColorFgNameBright(color),
            (true, false) => ColorBgName(color),
            (true, true) => ColorBgNameBright(color),
        };
        Some(sgr)
    }

    fn from_rgb(r: usize, g: usize, b: usize, background: bool) -> Option<Self> {
        let rgb = u32::try_from((r << 16) + (g << 8) + b).ok()?;
        let sgr = if background {
            Self::ColorBgRgb(rgb)
        } else {
            Self::ColorFgRgb(rgb)
        };
        Some(sgr)
    }

    fn color_rgb(mut params: impl Iterator<Item = usize>, background: bool) -> Option<Self> {
        match params.next()? {
            2 => {
                let (r, g, b) = (params.next()?, params.next()?, params.next()?);
                Self::from_rgb(r, g, b, background)
            }
            5 => {
                let n = params.next()?;
                match n {
                    0..=7 => Self::from_color_code(n, background, false),
                    8..=15 => Self::from_color_code(n - 8, background, true),
                    16..=231 => {
                        // palette represents a 6 * 6 * 6 cube where the three
                        // dimensions represent r, g, and b.
                        // Comments here assume a 2D representation of the cube.
                        // See: https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
                        const ROWS: usize = 6;
                        const COLUMNS: usize = 36;
                        const STEP_SIZE: usize = 0xFF / 6;

                        let n = n - 16;
                        // increases with each row
                        let r = (n / COLUMNS) * STEP_SIZE;
                        // g is constant for each 6 * 6 block
                        let g = ((n % COLUMNS) / ROWS) * STEP_SIZE;
                        // increases with each column but resets every 6.
                        let b = (n % ROWS) * STEP_SIZE;
                        Self::from_rgb(r, g, b, background)
                    }
                    232..=255 => {
                        const STEP_SIZE: usize = 0xFF / 24;
                        let n = n - 232;
                        Self::from_rgb(n * STEP_SIZE, n * STEP_SIZE, n * STEP_SIZE, background)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Parse a single SGR parameter from the parameters.
    /// This will only consume as many items from `params` as required to complete the SGR.
    fn from_params(mut params: impl Iterator<Item = usize>) -> Option<Self> {
        use Sgr::*;
        let code = params.next()?;
        Some(match code {
            0 => Reset,
            1 => Bold,
            2 => Dim,
            3 => Italic,
            4 => Underline,
            7 => Reverse,
            22 => BoldOff,
            23 => ItalicOff,
            24 => UnderlineOff,
            27 => ReverseOff,
            30..=37 => ColorFgName(ColorName::from_code(code)?),
            38 => Self::color_rgb(params, false)?,
            39 => ResetColorFg,
            40..=47 => ColorBgName(ColorName::from_code(code)?),
            48 => Self::color_rgb(params, true)?,
            49 => ResetColorBg,
            90..=97 => ColorFgNameBright(ColorName::from_code(code)?),
            100..=107 => ColorBgNameBright(ColorName::from_code(code)?),
            _ => return None,
        })
    }
}

/// Parse all SGR parameters in the given parameters.
/// This only consumes as many items from `params` as can be parsed by [`Sgr`].
pub(crate) fn parse_sgrs(mut params: impl Iterator<Item = usize>) -> Vec<Sgr> {
    iter::from_fn(|| Sgr::from_params(&mut params)).collect()
}

/// Describes the color effect of multiple SGR parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColorEffect {
    None,
    Name(ColorName),
    NameBright(ColorName),
    Rgb(u32),
}
impl ColorEffect {
    /// Get the 24-bit colour code.
    pub fn rgb(&self) -> Option<u32> {
        match self {
            Self::None => None,
            Self::Name(named) => Some(named.rgb(false)),
            Self::NameBright(named) => Some(named.rgb(true)),
            Self::Rgb(rgb) => Some(*rgb),
        }
    }
}
impl Default for ColorEffect {
    fn default() -> Self {
        Self::None
    }
}

impl From<&Sgr> for ColorEffect {
    fn from(sgr: &Sgr) -> Self {
        use Sgr::*;
        match sgr {
            ColorFgRgb(rgb) | ColorBgRgb(rgb) => Self::Rgb(*rgb),
            ColorFgName(name) | ColorBgName(name) => Self::Name(*name),
            ColorFgNameBright(name) | ColorBgNameBright(name) => Self::NameBright(*name),
            _ => Self::None,
        }
    }
}

/// Describes the effect that multiple SGR parameters have on text.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SgrEffect {
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub reverse: bool,
    /// Foreground colour
    pub fg: ColorEffect,
    /// Background colour
    pub bg: ColorEffect,
}
impl SgrEffect {
    fn reset(&mut self) {
        *self = Self::default();
    }

    /// Apply a SGR parameter to this effect.
    pub fn apply_sgr(&mut self, sgr: impl Borrow<Sgr>) {
        use Sgr::*;
        let sgr = sgr.borrow();
        match sgr {
            Reset => self.reset(),
            Bold => self.bold = true,
            Dim => self.dim = true,
            BoldOff => { self.bold = false; self.dim = false },
            Italic => self.italic = true,
            ItalicOff => self.italic = false,
            Underline => self.underline = true,
            UnderlineOff => self.underline = false,
            Reverse => self.reverse = true,
            ReverseOff => self.reverse = false,
            ColorFgRgb(_) | ColorFgName(_) | ColorFgNameBright(_) | ResetColorFg => {
                self.fg = ColorEffect::from(sgr);
            }
            ColorBgRgb(_) | ColorBgName(_) | ColorBgNameBright(_) | ResetColorBg => {
                self.bg = ColorEffect::from(sgr);
            }
        }
    }

    /// Apply multiple SGR parameters to this effect.
    pub fn apply_sgrs<T: Borrow<Sgr>>(&mut self, sgrs: impl IntoIterator<Item = T>) {
        for sgr in sgrs {
            self.apply_sgr(sgr.borrow());
        }
    }

    pub fn to_class_style<B: StyleBuilder>(&self) -> ClassStyle {
        let mut builder = B::default();
        if self.bold {
            builder.bold();
        }
        if self.italic {
            builder.italic();
        }
        if self.underline {
            builder.underline();
        }
        builder.fg_color(&self.fg);
        builder.bg_color(&self.bg);

        builder.finish()
    }
}
