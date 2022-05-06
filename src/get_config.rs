#[warn(non_snake_case)]
//to import enviorment variables from ecternal files
#[derive(serde::Deserialize)]
pub struct Settings {
    pub port: u16,
    pub host: String,
    pub dataBaseSetting: DataBase,
}
#[derive(serde::Deserialize)]
pub struct DataBase {
    pub url: String,
    pub name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config"))?;
    settings.try_into()
}
