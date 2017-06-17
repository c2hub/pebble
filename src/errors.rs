use ansi_term::Colour::Red;

use std::fmt::{Display, Formatter, Error};
use std::process::exit;
use std::error;

pub fn fail(msg: &str, code: i32) -> !
{
	eprintln!("  {} {}",
		Red.bold().paint("error"),
		msg
	);
	exit(code);
}

pub fn fail1<T: Display>(msg: &str, arg: T, code: i32) -> !
{
	eprintln!("  {} {}",
		Red.bold().paint("error"),
		msg.replace("{}", &arg.to_string())
	);
	exit(code);
}

#[derive(Clone, Debug)]
pub struct PebbleError
{
	pub msg: String,
}

impl PebbleError
{
	pub fn new(msg: &str) -> Self
	{
		PebbleError
		{
			msg: msg.to_owned()
		}
	}
}

impl Display for PebbleError
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		Ok({let _ = write!(f, "{}", self.msg);})
	}
}

impl error::Error for PebbleError
{
	fn description(&self) -> &str
	{
		self.msg.as_ref()
	}

	fn cause(&self) -> Option<&error::Error>
	{
		None
	}
}
