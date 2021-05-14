use crate::graphic_rendition::ColorEffect;
use std::borrow::Borrow;

/// Combination of classes and inline styles.
///
/// While it is possible to use inline styles only, it is not doable
/// with just classes due to the amount of RGB colour values.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassStyle {
    pub class: Option<String>,
    pub style: Option<String>,
}
impl ClassStyle {
    /// Push a new class to the classes.
    /// This function doesn't validate the given string in any way.
    pub fn push_class(&mut self, new: impl Borrow<str>) {
        const DELIMITER: char = ' ';
        let class = self.class.get_or_insert_with(String::new);
        if !class.is_empty() && !class.ends_with(DELIMITER) {
            class.push(DELIMITER);
        }
        class.push_str(new.borrow());
    }

    /// Push a new property to the style.
    /// This function doesn't validate the given string in any way.
    pub fn push_style(&mut self, new: impl Borrow<str>) {
        const DELIMITER: char = ';';
        let style = self.style.get_or_insert_with(String::new);
        if !style.is_empty() && !style.ends_with(DELIMITER) {
            style.push(DELIMITER);
        }
        style.push_str(new.borrow());
    }
}

/// Builder for [`ClassStyle`].
pub trait StyleBuilder: Default {
    /// Finish building and create a `ClassStyle`.
    fn finish(self) -> ClassStyle;

    /// Apply bold.
    fn bold(&mut self);
    /// Apply italic.
    fn italic(&mut self);
    /// Apply underline.
    fn underline(&mut self);

    /// Set the foreground colour.
    fn fg_color(&mut self, color: &ColorEffect);
    /// Set the background colour.
    fn bg_color(&mut self, color: &ColorEffect);
}

/// Style builder using only inline style attributes.
#[derive(Clone, Debug, Default)]
pub struct InlineStyle(ClassStyle);
impl InlineStyle {
    const CSS_BOLD: &'static str = "font-weight:bold;";
    const CSS_ITALIC: &'static str = "font-style:italic;";
    const CSS_UNDERLINE: &'static str = "text-decoration:underline;";
}
impl StyleBuilder for InlineStyle {
    fn finish(self) -> ClassStyle {
        self.0
    }

    fn bold(&mut self) {
        self.0.push_style(Self::CSS_BOLD);
    }

    fn italic(&mut self) {
        self.0.push_style(Self::CSS_ITALIC);
    }

    fn underline(&mut self) {
        self.0.push_style(Self::CSS_UNDERLINE);
    }

    fn fg_color(&mut self, color: &ColorEffect) {
        if let Some(code) = color.rgb() {
            self.0.push_style(format!("color:#{:06x};", code));
        }
    }

    fn bg_color(&mut self, color: &ColorEffect) {
        if let Some(code) = color.rgb() {
            self.0
                .push_style(format!("background-color:#{:06x};", code));
        }
    }
}
