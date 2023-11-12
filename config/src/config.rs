use configparser::ini::Ini;

pub struct Config {
    pub port: i64,

    pub top_left_x: f64,
    pub top_left_y: f64,
    pub bottom_right_x: f64,
    pub bottom_right_y: f64,

    pub capacity: i64
}

pub struct ConfigParsingError {
    pub details: String
}

impl ConfigParsingError {
    fn new(details: &str) -> ConfigParsingError {
        ConfigParsingError { details: details.to_string() }
    }
}

impl Config {
    pub fn parse(file: &str) -> Result<Config, ConfigParsingError> {
        let mut config = Ini::new();

        let result = config.load(file);
        let default_config = Config::default();

        match result {
            Ok(_) => {
                let returned_config = Config{
                    port: config.getint("server", "port").unwrap().unwrap_or(default_config.port),

                    top_left_x: config.getfloat("quadtree", "top_left_x").unwrap().unwrap_or(default_config.top_left_x),
                    top_left_y: config.getfloat("quadtree", "top_left_y").unwrap().unwrap_or(default_config.top_left_y),
                    bottom_right_x: config.getfloat("quadtree", "bottom_right_x").unwrap().unwrap_or(default_config.bottom_right_x),
                    bottom_right_y: config.getfloat("quadtree", "bottom_right_y").unwrap().unwrap_or(default_config.bottom_right_y),

                    capacity: config.getint("quadtree", "capacity").unwrap().unwrap_or(10)
                };

                return Ok(returned_config);
            }
            Err(_) => {
                return Err(ConfigParsingError::new(format!("Cannot parse config file: {file}").as_str()));
            }
        }
    }

    pub fn default() -> Config {
        return Config { 
            port: 50051, 
            top_left_x: 0.0,
            top_left_y: 0.0, 
            bottom_right_x: 10.0,
            bottom_right_y: 10.0,
            capacity: 10 
        }
    }

    pub fn parse_or_default(file: &str) -> Config {
        let parsed_config = Config::parse(file);

        match parsed_config {
            Ok(r) => r,
            Err(_) => Config::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn test_parse_full_config() {
        let config = Config::parse("mocks/full_config.ini");
        let config_result = config.ok().unwrap();

        assert_eq!(config_result.port, 5000);
        assert_eq!(config_result.top_left_x, 1.0);
        assert_eq!(config_result.top_left_y, 1.0);
        assert_eq!(config_result.bottom_right_x, 3.0);
        assert_eq!(config_result.bottom_right_y, 3.0);
        assert_eq!(config_result.capacity, 5);
    }

    #[test]
    fn test_parse_partial_config() {
        let config = Config::parse("mocks/partial_config.ini");
        let config_result = config.ok().unwrap();

        assert_eq!(config_result.port, 5000);
        assert_eq!(config_result.top_left_x, 1.0);
        assert_eq!(config_result.top_left_y, 0.0);
        assert_eq!(config_result.bottom_right_x, 3.0);
        assert_eq!(config_result.bottom_right_y, 10.0);
        assert_eq!(config_result.capacity, 5);
    }

    #[test]
    fn test_nonexistent_config() {
        let config = Config::parse("nonexistent_config.ini");

        assert!(config.is_err());
    }
}
