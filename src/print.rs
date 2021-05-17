use yew_ansi::{ColorEffect, SgrEffect};


pub fn print_options(effect: &SgrEffect) {
	print!("+");
	if effect.italic {
		print!("i");
	}
	if effect.underline {
		print!("u");
	}
	if effect.bold {
		print!("b");
	}
	if effect.reverse {
		print!("r");
	}
	if effect.dim {
		print!("d");
	}
}

pub fn print_color(color: &ColorEffect) {
	match color {
		ColorEffect::Name(color) => print!("{}", color),
		ColorEffect::NameBright(color) => print!("bright-{}", color),
		ColorEffect::Rgb(color) => print!("rgb:{:X}", color),
		ColorEffect::None => (),
	}
}
