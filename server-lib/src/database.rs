use sqlx::{
    types::Uuid,
    postgres, Row,
};
use rand::Rng;

use crate::error;

type Result<T> = std::result::Result<T, error::Error>;

pub struct DbUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub salt: [u8; 32],
    pub created_at: i32,
    pub connected_at: i32,
}

impl DbUser {
    fn new(user_id: Uuid, username: String, password: String,
        salt: [u8; 32], created_at: i32, connected_at: i32) -> Self {
        Self { user_id, username, password, salt, created_at, connected_at }
    }
}

pub struct _DbPassword {
    password_id: Uuid,
    user_id: Uuid,
    domain_name: String,
    username: String,
    password: Vec<u8>,
    nonce: [u8; 12],
}

impl _DbPassword {
    fn _new(password_id: Uuid, user_id: Uuid, domain_name: String,
        username: String, password: Vec<u8>, nonce: [u8; 12]) -> Self {
        Self { password_id, user_id, domain_name, username, password, nonce }
    }
}

pub struct _DbNote {
    note_id: Uuid,
    user_id: Uuid,
    title: Vec<u8>,
    content: Vec<u8>,
    nonce: [u8; 12],
}

impl _DbNote {
    fn _new(note_id: Uuid, user_id: Uuid, title: Vec<u8>,
        content: Vec<u8>, nonce: [u8; 12]) -> Self {
        Self {note_id, user_id, title, content, nonce }
    }
}

pub trait Db {
    async fn create_session(&self, user_id: &Uuid) -> Result<String>;
    async fn check_session(&self, session_id: String) -> Result<Uuid>;
    async fn create_user(&self, username: &String, password: &String) -> Result<()>;
    async fn check_user(&self, username: &String) -> Result<DbUser>;
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
    async fn create_session(&self, user_id: &Uuid) -> Result<String> {
        let sql = "
            INSERT INTO sessions (session_id, user_id) VALUES ($1, $2);
        ";
        let session_id = create_session_id();
        sqlx::query(sql)
            .bind(&session_id)
            .bind(&user_id)
            .execute(&self.pool)
            .await?;
        Ok(session_id)
    }
    async fn check_session(&self, session_id: String) -> Result<Uuid> {
        let sql = "SELECT user_id FROM sessions WHERE session_id = $1;";
        let query = sqlx::query_scalar(sql)
            .bind(session_id);
        let user_id = query
            .fetch_one(&self.pool)
            .await?;
        Ok(user_id)
    }

    async fn create_user(&self, username: &String, password: &String) -> Result<()> {
        let sql = "SELECT EXISTS (SELECT 1 FROM users WHERE users.username = $1);";
        let exists: bool = sqlx::query_scalar(sql)
            .bind(username.to_lowercase())
            .fetch_one(&self.pool)
            .await?;

        if exists {
            return Err(error::Error::DatabaseError)
        }

        let id = create_uuid_v4();
        let salt = create_salt();
        let timestamp = get_current_timestamp();
        let sql = "
            INSERT INTO users (user_id, username, password, salt, created_at, connected_at)
            VALUES ($1, $2, $3, $4, $5, $6);
        ";
        sqlx::query(sql)
            .bind(id)
            .bind(username.to_lowercase())
            .bind(&password)
            .bind(salt)
            .bind(timestamp)
            .bind(timestamp)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn check_user(&self, username: &String) -> Result<DbUser> {
        let sql = "SELECT * FROM users WHERE users.username = $1;";
        let query = sqlx::query(sql)
            .bind(username.to_lowercase());
        let row = query
            .fetch_one(&self.pool)
            .await?;
        Ok(DbUser::new(
            row.get("user_id"),
            row.get("username"),
            row.get("password"),
            row.get("salt"),
            row.get("created_at"),
            row.get("connected_at"),
        ))
    }
}

fn get_current_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

fn create_uuid_v4() -> Uuid {
    Uuid::new_v4()
}

fn create_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::thread_rng().fill(&mut salt);
    salt
}

fn create_session_id() -> String {
    let mut rng = rand::thread_rng();
    let session_id: String = (0..32)
        .map(|_| rng.gen_range(0..=15))
        .map(|n| format!("{:x}", n))
        .collect();
    session_id
}
