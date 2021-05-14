use std::env;
use std::fmt;
use std::io;

pub enum Error {
	ConfigVarError(env::VarError),
	IoError(io::Error),
	StarshipError(String)
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::ConfigVarError(ref err) => {
				match err {
					env::VarError::NotPresent => write!(f, "$kak_config should be exported and point to a directory containing starship.toml"),
					env::VarError::NotUnicode(_) => write!(f, "$kak_config value is not valid")
				}
			},
			Error::IoError(ref err) => write!(f, "Error executing starship {}", err),
			Error::StarshipError(ref err) => write!(f, "{}", err)
		}
	}
}

impl From<env::VarError> for Error {
	fn from(err: env::VarError) -> Error {
		Error::ConfigVarError(err)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::IoError(err)
	}
}
