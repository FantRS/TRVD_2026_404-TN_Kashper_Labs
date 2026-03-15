use deadpool_redis::Connection;
use deadpool_redis::Pool as RedisPool;
use deadpool_redis::redis::AsyncCommands as _;
use deadpool_redis::redis::FromRedisValue;
use deadpool_redis::redis::Pipeline;

use crate::app::{RequestError, RequestResult};

#[derive(Clone)]
pub struct RedisClient {
    pool: RedisPool,
}

impl RedisClient {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    async fn conn(&self) -> RequestResult<Connection> {
        self.pool.get().await.map_err(|e| {
            RequestError::internal_server_error(format!("Redis connection failed: {}", e))
        })
    }

    pub async fn get(&self, key: &str) -> RequestResult<Option<String>> {
        let mut conn = self.conn().await?;
        conn.get(key)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis GET: {}", e)))
    }

    pub async fn set(&self, key: &str, value: &str) -> RequestResult<()> {
        let mut conn = self.conn().await?;
        conn.set(key, value)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis SET: {}", e)))
    }

    pub async fn set_ex(&self, key: &str, value: &str, ttl: u64) -> RequestResult<()> {
        let mut conn = self.conn().await?;
        conn.set_ex(key, value, ttl)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis SET_EX: {}", e)))
    }

    pub async fn del(&self, key: &str) -> RequestResult<()> {
        let mut conn = self.conn().await?;
        conn.del(key)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis DEL: {}", e)))
    }

    pub async fn smembers(&self, key: &str) -> RequestResult<Vec<String>> {
        let mut conn = self.conn().await?;
        conn.smembers(key)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis SMEMBERS: {}", e)))
    }

    pub fn get_pipe(&self) -> Pipeline {
        deadpool_redis::redis::pipe()
    }

    pub async fn exec_pipe<T: FromRedisValue>(&self, pipeline: &Pipeline) -> RequestResult<T> {
        let mut conn = self.conn().await?;
        pipeline.query_async(&mut conn).await.map_err(|e| {
            RequestError::internal_server_error(format!("Redis Pipeline error: {}", e))
        })
    }

    /// XADD — додати запис в Redis Stream
    pub async fn xadd(&self, stream_key: &str, fields: &[(&str, &str)]) -> RequestResult<String> {
        let mut conn = self.conn().await?;

        let mut cmd = deadpool_redis::redis::cmd("XADD");
        cmd.arg(stream_key).arg("*");
        for (field, value) in fields {
            cmd.arg(*field).arg(*value);
        }

        let id: String = cmd
            .query_async(&mut conn)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis XADD: {}", e)))?;

        Ok(id)
    }

    /// XTRIM — обмежити довжину стріму (housekeeping)
    pub async fn xtrim(&self, stream_key: &str, max_len: usize) -> RequestResult<()> {
        let mut conn = self.conn().await?;

        deadpool_redis::redis::cmd("XTRIM")
            .arg(stream_key)
            .arg("MAXLEN")
            .arg("~")
            .arg(max_len)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| RequestError::internal_server_error(format!("Redis XTRIM: {}", e)))
    }
}
