use ansi_term::Colour::Red;

use std::fmt::Display;
use std::process::exit;

pub fn fail(msg: &str, code: i32) -> !
{
	println!("  {} {}",
		Red.bold().paint("error"),
		msg
	);
	exit(code);
}

pub fn fail1<T: Display>(msg: &str, arg: T, code: i32) -> !
{
	println!("  {} {}",
		Red.bold().paint("error"),
		msg.replace("{}", &arg.to_string())
	);
	exit(code);
}
