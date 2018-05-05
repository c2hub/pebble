#![feature(vec_remove_item)]

#[macro_use]
extern crate serde_derive;

extern crate ansi_term;
extern crate clap;
extern crate hyper;
extern crate pbr;
extern crate recipe_reader;
extern crate serde_cbor;
extern crate sha1;
extern crate toml;
extern crate version_compare;
extern crate walkdir;
extern crate zip;

mod commands;
mod config;
mod errors;
mod packets;
mod types;
mod util;

use commands::*;
use errors::fail1;
use types::PebbleType;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
	// Note: these versions mean percentage to desired functionality, rather than actual version
	let matches = App::new("pebble")
		.version("0.8.15")
		.author("Lukáš Hozda [magnusi] <luk.hozda@gmail.com>")
		.about("Pebble is a simple package and dependency manager for C2, akin to cargo.")
		.global_settings(&[AppSettings::ColoredHelp])
		.subcommand(SubCommand::with_name("new")
			.about("generates a new pebble")
			.version("1.0.0")
			.author("Lukáš Hozda")
			.arg(Arg::with_name("NAME")
				.help("name of the new pebble")
				.required(true)
				.index(1))
			.arg(Arg::with_name("TYPE")
				.help("type of the pebble to produce")
				.required(false)
				.index(2)))
		.subcommand(SubCommand::with_name("init")
			.about("initializes a pebble from existing source files in a existing directory")
			.version("1.0.0")
			.author("Lukáš Hozda")
			.arg(Arg::with_name("NAME")
				.help("path to the folder where to init the pebble")
				.required(true)
				.index(1))
			.arg(Arg::with_name("TYPE")
				.help("type of the pebble to produce")
				.required(false)
				.index(2)))
		.subcommand(SubCommand::with_name("scan")
			.about("scans src/ directory for new files and starts tracking them, removes files that do not exist anymore.")
			.version("0.9.5") // TODO update in git, too
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("add")
			.about("starts tracking a file")
			.version("0.9.5") // TODO add to git, too
			.author("Lukáš Hozda")
			.arg(Arg::with_name("FILENAME")
				.help("the file")
				.takes_value(true)
				.required(true)
				.index(1)))
		.subcommand(SubCommand::with_name("remove")
			.about("stops tracking a file")
			.version("0.9.5") // TODO remove from git, too
			.author("Lukáš Hozda")
			.arg(Arg::with_name("FILENAME")
				.help("the file")
				.takes_value(true)
				.required(true)
				.index(1)))
		.subcommand(SubCommand::with_name("build")
			.about("builds the pebble, producing output in the output/ directory")
			.version("0.6.0") // TODO check & fetch dependencies
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("run")
			.about("builds and runs the pebble, the program is ran in CWD, only valid for executable pebbles")
			.version("0.6.0") // TODO check & fetch dependencies
			.author("Lukáš Hozda")
			.arg(Arg::with_name("ARGS")
				.help("arguments to pass to the tests")
				.min_values(1)
				.required(false)))
		.subcommand(SubCommand::with_name("test")
			.about("builds and runs test of a pebble, only valid for library pebbles")
			.version("0.6.0") // TODO check & fetch dependencies
			.author("Lukáš Hozda")
			.arg(Arg::with_name("ARGS")
				.help("arguments to pass to the tests")
				.min_values(1)
				.required(false)))
		.subcommand(SubCommand::with_name("install")
			.about("runs install instructions of a pebble")
			.version("0.9.0") // Also allow downloading and installing of foreign pebbles, not just this one
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("uninstall")
			.about("runs uninstall instructions of a pebble")
			.version("0.9.0") // Also allow removal of foreign pebbles
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("update")
			.about("the fate of this command is unclear as of yet")
			.version("0.1.0") // Implement something meaningful. Either update dependencies or check if there is an update for a pebble
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("find")
			.about("tries to find a pebble in the pebble index, if found, prints the latest version")
			.version("1.0.0")
			.author("Lukáš Hozda")
			.arg(Arg::with_name("NAME")
				.help("the name of the pebble to find")
				.required(true)
				.index(1)))
		.subcommand(SubCommand::with_name("register")
			.about("register with pebble")
			.version("1.0.0")
			.author("Lukáš Hozda")
			.arg(Arg::with_name("USERNAME")
				.help("the username to register with")
				.required(true)
				.index(1))
			.arg(Arg::with_name("PASSWORD")
				.help("your password, there are no restrictions, but please, 1234 is a bad password")
				.required(true)
				.index(2)))
		.subcommand(SubCommand::with_name("login")
			.about("login with pebble, session only last until you restart your PC")
			.version("1.0.0")
			.author("Lukáš Hozda")
			.arg(Arg::with_name("USERNAME")
				.help("the username to login with")
				.required(true)
				.value_name("USERNAME")
				.takes_value(true)
				.index(1))
			.arg(Arg::with_name("PASSWORD")
				.help("your password")
				.value_name("PASSOWRD")
				.required(true)
				.takes_value(true)
				.index(2)))
		.subcommand(SubCommand::with_name("package")
			.about("package the pebble (confirm ability to be built, zip up)")
			.version("1.0.0")
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("upload")
			.about("upload a pebble to the pebble index, uploads a pebble as a whole, not like a library")
			.version("0.9.0") // Chop up bytes and send it as parts, not like one huge packet. That will also allow pebbles bigger than 60kb
			.author("Lukáš Hozda"))
		.subcommand(SubCommand::with_name("publish")
			.about("publish a pebble as a library, only uploads claim files in pebble.toml")
			.version("0.1.0")
			.author("Lukáš Hozda"))
		.get_matches();

	match matches.subcommand() {
		("new", Some(m)) => {
			if let Some(s) = m.value_of("TYPE") {
				if let Ok(t) = s.parse() {
					new_pebble(m.value_of("NAME").unwrap(), t);
				} else {
					new_pebble(m.value_of("NAME").unwrap(), PebbleType::Executable);
				}
			}
		}
		("init", Some(m)) => {
			if let Some(s) = m.value_of("TYPE") {
				match s.parse() {
					Ok(t) => init_pebble(m.value_of("NAME").unwrap(), t),
					Err(e) => fail1("{}", e, -1),
				}
			} else {
				init_pebble(m.value_of("NAME").unwrap(), PebbleType::Executable);
			}
		}
		("scan", _) => scan(),
		("add", Some(m)) => add(m.value_of("FILENAME").unwrap()),
		("remove", Some(m)) => remove(m.value_of("FILENAME").unwrap()),
		("build", _) => build(),
		("run", Some(m)) => run(m.values_of_lossy("ARGS").unwrap_or(Vec::new())),
		("test", Some(m)) => test(m.values_of_lossy("ARGS").unwrap_or(Vec::new())),
		("install", _) => install(),
		("uninstall", _) => uninstall(),
		("update", _) => update(),
		("find", Some(m)) => find(m.value_of("NAME").unwrap()),
		("register", Some(m)) => register(
			m.value_of("USERNAME").unwrap(),
			m.value_of("PASSWORD").unwrap(),
		),
		("login", Some(m)) => login(
			m.value_of("USERNAME").unwrap(),
			m.value_of("PASSWORD").unwrap(),
		),
		("package", _) => {
			let _ = package();
		}
		("upload", _) => upload(),
		("publish", _) => publish(),
		_ => unreachable!(),
	}
}
