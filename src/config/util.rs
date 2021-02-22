use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Config {
    config_path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    command_hash: String,
    command_path: PathBuf,
    args: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_config() {
        let path = "test_files/test_config.json";
        let path_buf = PathBuf::from(path);
        let config = Config::new(path_buf);
        let commands = config.get_config();
        println!("{:?}", commands);
    }
}

impl Config {
    pub fn new(config_path: PathBuf) -> Config {
        Config { config_path }
    }
    pub fn get_config(self) -> Result<Vec<Command>, Box<dyn Error>> {
        let file = File::open(self.config_path)?;
        let reader = BufReader::new(file);
        let commands: Vec<Command> = serde_json::from_reader(reader)?;
        Ok(commands)
    }
}
