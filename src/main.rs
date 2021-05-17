mod error;
mod escape;
mod print;

use crate::{
	error::Error,
	escape::{EscapeIterator, Token},
	print::{print_color, print_options},
};
use std::{env, path::Path, process::Command};
use yew_ansi::{get_sgr_segments, ColorEffect};


fn main() -> Result<(), Error> {
	let config_dir = env::var("kak_config")?;
	let config = Path::new(&config_dir).join("starship.toml");
	let args: Vec<String> = env::args().skip(1).collect();
	let starship = Command::new("starship")
		.env("STARSHIP_SHELL", "sh")
		.env("STARSHIP_CONFIG", config)
		.args(&args)
		.output()?;

	return if starship.status.code() != Some(0) {
		Err(Error::StarshipError(
			String::from_utf8_lossy(&starship.stderr).into(),
		))
	} else {
		let stdout = String::from_utf8_lossy(&starship.stdout);
		if args.first().filter(|v| *v == "prompt").is_some() {
			for (effect, txt) in get_sgr_segments(&stdout) {
				let has_option = effect.italic
					|| effect.underline || effect.bold
					|| effect.reverse || effect.dim;
				let has_color = effect.bg != ColorEffect::None || effect.fg != ColorEffect::None;
				if has_option || has_color {
					let has_colors =
						effect.bg != ColorEffect::None && effect.fg != ColorEffect::None;
					print!("{{");
					print_color(&effect.fg);
					if has_colors {
						print!(",");
					}
					print_color(&effect.bg);
					if has_option {
						print_options(&effect);
					}
					print!("}}");
				}
				for token in EscapeIterator::new(txt) {
					match token {
						Token::Percent => print!("%%"),
						Token::Str(txt) => print!("{}", txt),
						Token::Block(txt) => print!("{}", txt),
					}
				}
			}
		} else {
			println!("{}", stdout);
			eprintln!("{}", String::from_utf8_lossy(&starship.stderr));
		}
		Ok(())
	};
}
