#![feature(vec_remove_item)]

extern crate recipe_reader;
extern crate serde_cbor;
extern crate ansi_term;
extern crate walkdir;
extern crate hyper;
extern crate serde;
extern crate toml;
extern crate sha1;
extern crate zip;

#[macro_use]
extern crate serde_derive;

mod commands;
mod register;
mod packets;
mod config;
mod upload;
mod login;
mod types;
mod build;
mod util;

mod package;

use commands::*;
use register::*;
use package::*;
use upload::*;
use build::*;
use login::*;
use types::*;

use std::process::exit;
use std::env::args;


fn main()
{
	let arguments: Vec<String> = args().collect();

	if arguments.len() < 2
		{help(&String::new());}

	match arguments[1].as_ref()
	{
		"new" =>
		{
			match arguments.len()
			{
				3 => new_pebble(&arguments[2], PebbleType::Executable),
				4 => match arguments[3].as_ref()
				{
					"lib"|
					"libstatic"|
					"staticlib" =>
						new_pebble(&arguments[2], PebbleType::StaticLib),
					"dynamic"|
					"dynamiclib"|
					"sharedlib"|
					"libshared"|
					"shared" =>
						new_pebble(&arguments[2], PebbleType::SharedLib),
					"executable"|
					"bin"|
					"binary"|
					"exe" =>
						new_pebble(&arguments[2], PebbleType::Executable),
					x =>
					{
						println!("unknown pebble type '{}'", x);
						exit(-1);
					}
				},
				_ =>
				{
					println!("usage: pebble new <name> <type>");
					exit(-1);
				},
			}
		}
		"init" => match arguments.len()
		{
			3 => init_pebble(&arguments[2], PebbleType::Executable),
			4 => match arguments[3].as_ref()
			{
				"lib"|
				"libstatic"|
				"staticlib"|
				"static"|
				"slib" =>
					init_pebble(&arguments[2], PebbleType::StaticLib),
				"dynamic"|
				"dynamiclib"|
				"sharedlib"|
				"libshared"|
				"shared"|
				"dlib" =>
					init_pebble(&arguments[2], PebbleType::SharedLib),
				"executable"|
				"bin"|
				"binary"|
				"exe" =>
					init_pebble(&arguments[2], PebbleType::Executable),
				x =>
				{
					println!("unknown pebble type '{}'", x);
					exit(-1);
				}
			},
			_ =>
			{
				println!("usage: pebble new <name> <type>");
				exit(-1);
			},
		},
		"scan" => match arguments.len()
		{
			2 => scan(),
			_ =>
			{
				println!("usage: pebble scan #what else?");
				exit(-1);
			}
		},
		"add" => match arguments.len()
		{
			3 => add(&arguments[2]),
			_ =>
			{
				println!("usage: pebble add 'filename'");
				exit(-1);
			}
		},
		"remove" | "del" => match arguments.len()
		{
			3 => remove(&arguments[2]),
			_ =>
			{
				println!("usage: pebble remove 'filename'");
				exit(-1);
			}
		},
		"build" => match arguments.len()
		{
			2 => build(),
			_ =>
			{
				println!("usage: pebble build");
				exit(-1);
			}
		},
		"help" => match arguments.len()
		{
			2 => help(&String::new()),
			_ => help(&arguments[2].to_string()),
		},
		"run" => run(arguments.clone().into_iter().skip(2).collect()),
		"test" => test(arguments.clone().into_iter().skip(2).collect()),
		"install" => install(),
		"uninstall" => uninstall(),
		"update" => update(),
		"find" => match arguments.len()
		{
			3 => find(&arguments[2].to_string()),
			_ => help(&String::new()),
		},
		"register" => match arguments.len()
		{
			4 => register(&arguments[2].to_string(), &arguments[3].to_string()),
			_ => help(&String::new())
		},
		"login" => match arguments.len()
		{
			4 => login(&arguments[2].to_string(), &arguments[3].to_string()),
			_ => help(&String::new())
		},
		"package" => { let _ = package(); }
		"upload" => upload(),
		x =>
		{
			println!("unknown operation: '{}'", x);
			exit(-1);
		}
	}
}
