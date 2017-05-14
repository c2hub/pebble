
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
	pub source_dir: Option<String>,
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
