use std::time::SystemTime;

use deadpool_postgres::{Client, Config, ManagerConfig, Pool, PoolError, RecyclingMethod};
use deadpool_postgres::tokio_postgres::NoTls;
use serde::{Deserialize, Serialize};

use crate::config::PostgresConfig;

pub struct Postgres {}

impl Postgres {
    pub fn new_pool(config: PostgresConfig) -> Pool {
        let mut cfg = Config::new();

        cfg.host = Some(config.host);
        cfg.port = Some(config.port);
        cfg.dbname = Some(config.dbname);
        cfg.user = Some(config.user);
        cfg.password = Some(config.password);

        // FIXME more settings
        // PG__POOL__MAX_SIZE=16
        // PG__POOL__TIMEOUTS__WAIT__SECS=5
        // PG__POOL__TIMEOUTS__WAIT__NANOS=0

        cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

        let pool = cfg.create_pool(NoTls).unwrap();

        pool
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sensor {
    pub temperature: i32,
    pub time: SystemTime,
}

const SELECT_QUERY: &str = r"
        SELECT temperature, time
            FROM temperature_sensors
            WHERE sensor_id = $1
            ORDER BY time DESC
            LIMIT 1";

pub async fn get_latest_temperature(pool: &Pool, sensor_id: i32) -> Result<Sensor, PoolError> {
    let client: Client = pool.get().await?;
    let stmt = client.prepare_cached(SELECT_QUERY).await?;
    let row = client.query_one(&stmt, &[&sensor_id]).await?;
    Ok(Sensor { temperature: row.get(0), time: row.get(1) })
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

        let pool = Postgres::new_pool(postgres_config);

        let sensor = get_latest_temperature(&pool, 0)
            .await
            .unwrap();

        println!("{:?}", sensor);

        assert_eq!(sensor.temperature, 0);
    }
}
