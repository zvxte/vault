use sqlx::{
    types::Uuid,
    postgres, Row,
};
use rand::Rng;
use crypto::hash_with_sha3;
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

pub struct DbPassword {
    pub password_id: Uuid,
    pub user_id: Uuid,
    pub domain_name: String,
    pub username: String,
    pub password: Vec<u8>,
    pub nonce: [u8; 12],
}

impl DbPassword {
    fn new(password_id: Uuid, user_id: Uuid, domain_name: String,
        username: String, password: Vec<u8>, nonce: [u8; 12]) -> Self {
        Self { password_id, user_id, domain_name, username, password, nonce }
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
    fn _new(note_id: Uuid, user_id: Uuid, title: Vec<u8>, title_nonce: [u8; 12],
        content: Vec<u8>, content_nonce: [u8; 12]) -> Self {
        Self {note_id, user_id, title, title_nonce, content, content_nonce }
    }
}

pub trait Db {
    async fn create_session(&self, user_id: &Uuid) -> Result<String>;
    async fn validate_session(&self, session_id: &String) -> Result<Uuid>;
    async fn create_user(&self, username: &String, password: &String) -> Result<()>;
    async fn get_user(&self, username: &String) -> Result<DbUser>;
    async fn create_password(&self, user_id: &Uuid, domain_name: &String,
        username: &String, password: &Vec<u8>, nonce: &[u8; 12]) -> Result<DbPassword>;
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
    async fn create_session(&self, user_id: &Uuid) -> Result<String> {
        let sql = "
            INSERT INTO sessions (session_id, user_id) VALUES ($1, $2);
        ";
        let session_id = create_session_id();
        let hashed_session_id = hash_with_sha3(&session_id);
        sqlx::query(sql)
            .bind(hashed_session_id)
            .bind(&user_id)
            .execute(&self.pool)
            .await?;
        Ok(session_id)
    }
    async fn validate_session(&self, session_id: &String) -> Result<Uuid> {
        let hashed_session_id = hash_with_sha3(session_id);
        let sql = "SELECT user_id FROM sessions WHERE sessions.session_id = $1;";
        let query = sqlx::query_scalar(sql)
            .bind(hashed_session_id);
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

    async fn get_user(&self, username: &String) -> Result<DbUser> {
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

    async fn create_password(&self, user_id: &Uuid, domain_name: &String,
            username: &String, password: &Vec<u8>, nonce: &[u8; 12]) -> Result<DbPassword> {
        let password_id = create_uuid_v4();
        let sql = "
            INSERT INTO passwords 
            (password_id, user_id, domain_name, username, password, nonce) 
            VALUES ($1, $2, $3, $4, $5, $6);
        ";
        sqlx::query(sql)
            .bind(&password_id)
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
        let query = sqlx::query(sql)
            .bind(user_id)
            .bind(password_id);
        let row = query
            .fetch_one(&self.pool)
            .await?;
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
        let query = sqlx::query(sql)
            .bind(user_id);
        let rows = query
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| { DbPassword::new(
                row.get("password_id"),
                row.get("user_id"),
                row.get("domain_name"),
                row.get("username"),
                row.get("password"),
                row.get("nonce"),
            ) })
            .collect()
        )
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
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map( char::from )
        .collect()
}
