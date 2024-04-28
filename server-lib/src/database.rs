use crate::error;
use sqlx::{postgres, types::Uuid, Row};

type Result<T> = std::result::Result<T, error::Error>;

pub struct DbUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub salt: [u8; 32],
    pub created_at: i64,
    pub connected_at: i64,
}

impl DbUser {
    fn new(
        user_id: Uuid,
        username: String,
        password: String,
        salt: [u8; 32],
        created_at: i64,
        connected_at: i64,
    ) -> Self {
        Self {
            user_id,
            username,
            password,
            salt,
            created_at,
            connected_at,
        }
    }
}

pub struct DbPassword {
    pub password_id: Uuid,
    pub user_id: Uuid,
    pub domain_name: String,
    pub username: String,
    pub password: Vec<u8>,
    pub nonce: [u8; 12],
}

impl DbPassword {
    fn new(
        password_id: Uuid,
        user_id: Uuid,
        domain_name: String,
        username: String,
        password: Vec<u8>,
        nonce: [u8; 12],
    ) -> Self {
        Self {
            password_id,
            user_id,
            domain_name,
            username,
            password,
            nonce,
        }
    }
}

pub struct _DbNote {
    note_id: Uuid,
    user_id: Uuid,
    title: Vec<u8>,
    title_nonce: [u8; 12],
    content: Vec<u8>,
    content_nonce: [u8; 12],
}

impl _DbNote {
    fn _new(
        note_id: Uuid,
        user_id: Uuid,
        title: Vec<u8>,
        title_nonce: [u8; 12],
        content: Vec<u8>,
        content_nonce: [u8; 12],
    ) -> Self {
        Self {
            note_id,
            user_id,
            title,
            title_nonce,
            content,
            content_nonce,
        }
    }
}

pub trait Db {
    async fn create_session(&self, hashed_session_id: &[u8; 32], user_id: &Uuid) -> Result<()>;
    async fn validate_session(&self, hashed_session_id: &[u8; 32]) -> Result<Uuid>;
    async fn delete_session(&self, hashed_session_id: &[u8; 32]) -> Result<()>;
    async fn create_user(
        &self,
        user_id: &Uuid,
        username: &String,
        password: &String,
        salt: &[u8; 32],
        created_at: i64,
        connected_at: i64,
    ) -> Result<()>;
    async fn get_user(&self, username: &String) -> Result<DbUser>;
    async fn update_user_timestamp(&self, user_id: &Uuid, connected_at: i64) -> Result<()>;
    async fn create_password(
        &self,
        password_id: &Uuid,
        user_id: &Uuid,
        domain_name: &String,
        username: &String,
        password: &Vec<u8>,
        nonce: &[u8; 12],
    ) -> Result<DbPassword>;
    async fn get_password(&self, user_id: &Uuid, password_id: &Uuid) -> Result<DbPassword>;
    async fn get_passwords(&self, user_id: &Uuid) -> Result<Vec<DbPassword>>;
    async fn delete_password(&self, user_id: &Uuid, password_id: &Uuid) -> Result<()>;
}

#[derive(Clone)]
pub struct PostgreDb {
    pool: postgres::PgPool,
}

impl PostgreDb {
    pub async fn build(url: String) -> Result<Self> {
        let pool = postgres::PgPool::connect(&url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}

impl Db for PostgreDb {
    async fn create_session(&self, hashed_session_id: &[u8; 32], user_id: &Uuid) -> Result<()> {
        let sql = "
            INSERT INTO sessions (session_id, user_id) VALUES ($1, $2);
        ";
        sqlx::query(sql)
            .bind(hashed_session_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn validate_session(&self, hashed_session_id: &[u8; 32]) -> Result<Uuid> {
        let sql = "SELECT user_id FROM sessions WHERE sessions.session_id = $1;";
        let query = sqlx::query_scalar(sql).bind(hashed_session_id);
        let user_id = query.fetch_one(&self.pool).await?;
        Ok(user_id)
    }

    async fn delete_session(&self, hashed_session_id: &[u8; 32]) -> Result<()> {
        let sql = "DELETE FROM sessions WHERE sessions.session_id = $1;";
        sqlx::query(sql)
            .bind(hashed_session_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_user(
        &self,
        user_id: &Uuid,
        username: &String,
        password: &String,
        salt: &[u8; 32],
        created_at: i64,
        connected_at: i64,
    ) -> Result<()> {
        let sql = "SELECT EXISTS (SELECT 1 FROM users WHERE users.username = $1);";
        let exists: bool = sqlx::query_scalar(sql)
            .bind(username.to_lowercase())
            .fetch_one(&self.pool)
            .await?;

        if exists {
            return Err(error::Error::DatabaseError);
        }

        let sql = "
            INSERT INTO users (user_id, username, password, salt, created_at, connected_at)
            VALUES ($1, $2, $3, $4, $5, $6);
        ";
        sqlx::query(sql)
            .bind(user_id)
            .bind(username.to_lowercase())
            .bind(password)
            .bind(salt)
            .bind(created_at)
            .bind(connected_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_user(&self, username: &String) -> Result<DbUser> {
        let sql = "SELECT * FROM users WHERE users.username = $1;";
        let query = sqlx::query(sql).bind(username.to_lowercase());
        let row = query.fetch_one(&self.pool).await?;
        Ok(DbUser::new(
            row.get("user_id"),
            row.get("username"),
            row.get("password"),
            row.get("salt"),
            row.get("created_at"),
            row.get("connected_at"),
        ))
    }

    async fn update_user_timestamp(&self, user_id: &Uuid, connected_at: i64) -> Result<()> {
        let sql = "UPDATE users SET users.connected_at = $1 WHERE users.user_id = $2;";
        sqlx::query(sql)
            .bind(connected_at)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_password(
        &self,
        password_id: &Uuid,
        user_id: &Uuid,
        domain_name: &String,
        username: &String,
        password: &Vec<u8>,
        nonce: &[u8; 12],
    ) -> Result<DbPassword> {
        let sql = "
            INSERT INTO passwords
            (password_id, user_id, domain_name, username, password, nonce)
            VALUES ($1, $2, $3, $4, $5, $6);
        ";
        sqlx::query(sql)
            .bind(password_id)
            .bind(user_id)
            .bind(domain_name)
            .bind(username)
            .bind(password)
            .bind(nonce)
            .execute(&self.pool)
            .await?;
        self.get_password(&user_id, &password_id).await
    }

    async fn get_password(&self, user_id: &Uuid, password_id: &Uuid) -> Result<DbPassword> {
        let sql = "
            SELECT * FROM passwords WHERE
            passwords.user_id = $1 AND passwords.password_id = $2;
        ";
        let query = sqlx::query(sql).bind(user_id).bind(password_id);
        let row = query.fetch_one(&self.pool).await?;
        Ok(DbPassword::new(
            row.get("password_id"),
            row.get("user_id"),
            row.get("domain_name"),
            row.get("username"),
            row.get("password"),
            row.get("nonce"),
        ))
    }

    async fn get_passwords(&self, user_id: &Uuid) -> Result<Vec<DbPassword>> {
        let sql = "SELECT * FROM passwords WHERE passwords.user_id = $1;";
        let query = sqlx::query(sql).bind(user_id);
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(|row| {
                DbPassword::new(
                    row.get("password_id"),
                    row.get("user_id"),
                    row.get("domain_name"),
                    row.get("username"),
                    row.get("password"),
                    row.get("nonce"),
                )
            })
            .collect())
    }

    async fn delete_password(&self, user_id: &Uuid, password_id: &Uuid) -> Result<()> {
        let sql = "
            DELETE from passwords WHERE passwords.user_id = $1 AND passwords.password_id = $2;
        ";
        sqlx::query(sql)
            .bind(user_id)
            .bind(password_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
