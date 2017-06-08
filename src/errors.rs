use ansi_term::Colour::Red;

use std::process::exit;
use std::fmt::Display;

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
