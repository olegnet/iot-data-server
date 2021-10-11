use std::fs;
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub const DEFAULT_CONFIG_NAME: &str = "config.json";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename(deserialize = "bind"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<Vec<SocketAddr>>,
    #[serde(rename(deserialize = "postgres"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgres: Option<PostgresConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresConfig {
    #[serde(rename(deserialize = "host"))]
    pub host: String,
    #[serde(rename(deserialize = "port"))]
    pub port: u16,
    #[serde(rename(deserialize = "dbname"))]
    pub dbname: String,
    #[serde(rename(deserialize = "user"))]
    pub user: String,
    #[serde(rename(deserialize = "password"))]
    pub password: String,
}

impl Config {
    pub fn read_from(name: String) -> std::io::Result<Config> {
        let buf = fs::read(name)?;
        let config = serde_json::from_slice(buf.as_slice())?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use SocketAddr::{V4, V6};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    use crate::config::{Config, PostgresConfig};

    fn provide_config() -> Config {
        Config {
            bind: Some(vec![
                V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)),
                V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0)),
            ]),
            postgres: Some(PostgresConfig {
                host: "192.168.1.100".to_string(),
                port: 5432,
                dbname: "main".to_string(),
                user: "iot-data".to_string(),
                password: "123 (not really)".to_string(),
            }),
        }
    }

    fn provide_json() -> String {
        r#"{"bind":["127.0.0.1:8080","[::1]:8080"],"postgres":{"host":"192.168.1.100","port":5432,"dbname":"main","user":"iot-data","password":"123 (not really)"}}"#.to_string()
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
        let json = Config::read_from("config-example.json".to_string()).unwrap();
        assert_eq!(config, json);
    }

    #[test]
    fn read_config_failed_test() {
        Config::read_from("some file".to_string()).unwrap_err();
    }
}
