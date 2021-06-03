mod error;

use crate::error::Error;
use std::{env, path::Path, process::Command};
use kak::{face, escape::Mode};

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
			face::print(&stdout, Mode::Block);
		} else {
			println!("{}", stdout);
			eprintln!("{}", String::from_utf8_lossy(&starship.stderr));
		}
		Ok(())
	};
}
