use std::time::SystemTime;

use deadpool_postgres::{Client, Config, ManagerConfig, Pool, PoolError, RecyclingMethod};
use deadpool_postgres::config::ConfigError;
use deadpool_postgres::tokio_postgres::NoTls;
use serde::{Deserialize, Serialize};

use crate::config::PostgresConfig;

pub struct Postgres {}

impl Postgres {
    pub fn new_pool(config: PostgresConfig) -> Result<Pool, ConfigError> {
        let mut cfg = Config::new();
        cfg.host = Some(config.host);
        cfg.port = Some(config.port);
        cfg.dbname = Some(config.dbname);
        cfg.user = Some(config.user);
        cfg.password = Some(config.password);
        cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

        cfg.create_pool(NoTls)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sensor {
    pub temperature: f32,
    pub time: SystemTime,
}

const SELECT_QUERY: &str = r"
        SELECT temperature, time
            FROM temperature_sensors
            WHERE sensor_id = $1
            ORDER BY time DESC
            LIMIT 1";

const INSERT_QUERY: &str = r"
        INSERT INTO temperature_sensors (sensor_id, temperature, time)
            VALUES ($1, $2, now())";

pub async fn get_latest_temperature(pool: &Pool, sensor_id: i32) -> Result<Sensor, PoolError> {
    let client: Client = pool.get().await?;
    let stmt = client.prepare_cached(SELECT_QUERY).await?;
    let row = client.query_one(&stmt, &[&sensor_id]).await?;
    Ok(Sensor { temperature: row.get(0), time: row.get(1) })
}

pub async fn insert_temperature(pool: &Pool, sensor_id: i32, temperature: f32) -> Result<u64, PoolError> {
    let client: Client = pool.get().await?;
    let stmt = client.prepare_cached(INSERT_QUERY).await?;
    let result = client.execute(&stmt, &[&sensor_id, &temperature]).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::config::DEFAULT_CONFIG_NAME;
    use crate::postgres::{get_latest_temperature, Postgres};

    #[tokio::test]
    async fn get_latest_temperature_test() {
        let postgres_config = crate::config::Config::read_from(DEFAULT_CONFIG_NAME.to_string())
            .unwrap()
            .postgres
            .unwrap();

        let pool = Postgres::new_pool(postgres_config)
            .unwrap();

        let sensor = get_latest_temperature(&pool, 0)
            .await
            .unwrap();

        println!("{:?}", sensor);

        assert_eq!(sensor.temperature, 0.);
    }
}
