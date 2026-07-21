use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum DomainError {
  #[error("invalid configuration parameter: {0}")]
  ConfigError(String),
/*
  #[error("database query failed")]
  DatabaseError {
    #[source]
    source: sqlx::Error, // Keeps track of lower-level causes
  },
*/
  #[error("input/output failure occurred")]
  IoError(#[from] io::Error), // Automatically converts std::io::Error to DomainError via `?`

  #[error("requested resource (ID {id}) was not found")]
  NotFound { id: u64 },

  #[error("failed to convert digit for radix {0}")]
  InvalidDigit(u8),
}
