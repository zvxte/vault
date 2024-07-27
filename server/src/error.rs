use sqlx;

#[derive(Debug)]
pub enum Error {
    DatabaseError,
    SqlxError(sqlx::Error),
    SqlxMigrateError(sqlx::migrate::MigrateError),
    HeadersError,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::SqlxError(value)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Error::SqlxMigrateError(value)
    }
}
