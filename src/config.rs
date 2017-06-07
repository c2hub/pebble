use toml;
use std::fs::File;
use std::io::Read;

use errors::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config
{
	pub pebble: PackageCfg,
	pub lib: Option<LibCfg>,
	pub build: Option<BuildCfg>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PackageCfg
{
	pub name: String,
	pub version: String,
	pub source_dir: Option<String>,	//TODO
	pub license: Option<String>,
	pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency
{
	pub name: String,
	pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibCfg
{
	pub claim: Vec<String>,
	pub extra: Vec<String>,
	pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildCfg
{
	pub pre: Option<Vec<String>>,
	pub post: Option<Vec<String>>,
	pub install: Option<Vec<String>>,
	pub uninstall: Option<Vec<String>>
}

#[allow(dead_code)]
impl Config
{
	pub fn read() -> Result<Config, toml::de::Error>
	{
		let mut me = match File::open("pebble.toml")
		{
			Ok(f) => f,
			Err(_) => fail("failed to open pebble.toml", 7)
		};
		let mut contents = String::new();
		if me.read_to_string(&mut contents).is_err()
			{fail("failed to read pebble.toml", 8)}
		toml::from_str(contents.as_ref())
	}

	pub fn write(&self) -> Result<String, toml::ser::Error>
	{
		toml::to_string(&self)
	}
}
