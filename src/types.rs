use std::str::FromStr;

use errors::PebbleError;

#[derive(Clone, Copy)]
pub enum PebbleType
{
	Executable,
	SharedLib,
	StaticLib,
}

impl FromStr for PebbleType
{
	type Err = PebbleError;
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s
		{
			"lib"|
			"libstatic"|
			"staticlib" =>
				Ok(PebbleType::StaticLib),
			"dynamic"|
			"dynamiclib"|
			"sharedlib"|
			"libshared"|
			"shared" =>
				Ok(PebbleType::SharedLib),
			"executable"|
			"bin"|
			"binary"|
			"exe" =>
				Ok(PebbleType::StaticLib),
			x => Err(PebbleError::new(format!("invalid pebble type string '{}'", x).as_ref()))
		}
	}
}

impl ToString for PebbleType
{
	fn to_string(&self) -> String
	{
		match *self
		{
			PebbleType::Executable => "executable".to_string(),
			PebbleType::SharedLib =>  "shared library".to_string(),
			PebbleType::StaticLib => "static library".to_string()
		}
	}
}


#[derive(Clone, Serialize, Deserialize)]
pub struct User
{
	pub name: String,
	pub hash: String,
}
