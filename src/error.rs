use nix;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
	System,
	InvalidPath,

	NoSize,
	WrongSize,
}

impl From<nix::Error> for Error {
	fn from(error: nix::Error) -> Error {
		match error {
			nix::Error::Sys(errno) => {
				match errno {
					_ => Error::System
				}
			}

			nix::Error::InvalidPath => {
				Error::InvalidPath
			}
		}
	}
}
