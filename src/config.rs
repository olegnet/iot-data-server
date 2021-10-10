use std::fs;

use serde::{Deserialize, Serialize};

pub const DEFAULT_CONFIG_NAME: &str = "config.json";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename(deserialize = "bind"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<Vec<Bind>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Bind {
    #[serde(rename(deserialize = "address"))]
    address: Option<String>,
    #[serde(rename(deserialize = "port"))]
    port: Option<u16>,
}

impl Config {
    pub fn read_from(name: String) -> std::io::Result<Config> {
        let buf = fs::read(name)?;
        let config = serde_json::from_slice(buf.as_slice())?;
        Ok(config)
    }
}

impl Bind {
    pub fn get(&self) -> String {
        let address = match self.address.as_ref() {
            Some(v) => v,
            None => "",
        };
        let port = self.port.unwrap_or(0);

        format!("{}:{}", address, port)
    }
}


#[cfg(test)]
mod tests {
    use crate::config::{Bind, Config, DEFAULT_CONFIG_NAME};

    fn provide_config() -> Config {
        Config {
            bind: Some(vec![
                Bind {
                    address: Some("127.0.0.1".to_string()),
                    port: Some(8080),
                },
                Bind {
                    address: Some("[::1]".to_string()),
                    port: Some(8080),
                },
            ]),
        }
    }

    fn provide_json() -> String {
        r#"{"bind":[{"address":"127.0.0.1","port":8080},{"address":"[::1]","port":8080}]}"#.to_string()
    }

    #[test]
    fn serialize_config_test() {
        let config = provide_config();
        let json = serde_json::to_string(&config).unwrap();
        assert_eq!(provide_json(), json);
    }

    #[test]
    fn deserialize_config_test() {
        let json = provide_json();
        let config: Config = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(provide_config(), config);
    }

    #[test]
    fn read_config_test() {
        let config = provide_config();
        let json = Config::read_from(DEFAULT_CONFIG_NAME.to_string()).unwrap();
        assert_eq!(config, json);
    }

    #[test]
    fn read_config_failed_test() {
        let error = Config::read_from("some file".to_string()).unwrap_err();
        eprintln!("{:?}", error);
    }
}
