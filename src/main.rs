extern crate recipe_reader;
extern crate serde;
extern crate toml;

#[macro_use]
extern crate serde_derive;

mod commands;
mod config;
mod types;

use commands::*;
use types::*;

use std::process::exit;
use std::env::args;

fn main()
{
	let arguments: Vec<String> = args().collect();
	match arguments[1].as_ref()
	{
		"new" =>
		{
			match arguments.len()
			{
				2 => new_pebble(&arguments[2], PebbleType::Executable),
				_ => {},
			}
		}
		x =>
		{
			println!("unknown operation: '{}'", x);
			exit(-1);
		}
	}
}
