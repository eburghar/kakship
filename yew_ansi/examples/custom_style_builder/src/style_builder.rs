use yew_ansi::{ClassStyle, ColorEffect, StyleBuilder};

// it's required that the builder implements `Default`.
#[derive(Default)]
pub struct MyStyleBuilder(ClassStyle);
impl StyleBuilder for MyStyleBuilder {
    fn finish(self) -> ClassStyle {
        let mut style = self.0;
        // add the "ansi" class to every segment.
        // we could also use a custom `Default` implementation which adds this class at the start.
        style.push_class("ansi");
        style
    }

    fn bold(&mut self) {
        self.0.push_class("bold");
    }
    fn italic(&mut self) {
        self.0.push_class("italic");
    }
    fn underline(&mut self) {
        self.0.push_class("underline");
    }

    fn fg_color(&mut self, color: &ColorEffect) {
        // we could use classes for the named colours but it's simpler to just pass the hex colour.
        if let Some(code) = color.rgb() {
            self.0.push_style(format!("--fg-color:#{:06x}", code));
        }
    }
    fn bg_color(&mut self, color: &ColorEffect) {
        if let Some(code) = color.rgb() {
            self.0.push_style(format!("--bg-color:#{:06x}", code));
        }
    }
}
