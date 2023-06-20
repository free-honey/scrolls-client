use thiserror::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Redis Error: {0}")]
    Redis(redis::RedisError),
}
