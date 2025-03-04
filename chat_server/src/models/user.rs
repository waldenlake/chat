use std::mem;

use crate::error::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use super::User;
use anyhow::Result;

impl User {
    /// find s user by email
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user =
         sqlx::query_as("SELECT id, fullname, email, password_hash, created_at FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
            Ok(user)
    }

    /// create a user
    pub async fn create(fullname: &str, email: &str, password: &str, pool: &PgPool) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;
        let user =
            sqlx::query_as(
            r#"
            INSERT INTO users (fullname, email, password_hash)
            VALUES($1, $2, $3)
            RETURNING id, fullname, email, created_at
            "#
        )
        .bind(fullname)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn verify(email: &str, password: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> =
            sqlx::query_as("SELECT id, fullname, email, password_hash, created_at FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                }else{
                    Ok(None)
                }
            }
            None => Ok(None)
        }
    }

    
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;
    let is_valid = argon2.verify_password(password.as_bytes(), &password_hash).is_ok();
    Ok(is_valid)
}


#[cfg(test)]
mod tests {
    use crate::models::{user::verify_password, User};
    use anyhow::Result;
    use super::hash_password;
    use sqlx_db_tester::TestPg;
    use std::path::Path;

    #[test]
    fn hash_password_and_verify_should_work() -> Result<()>{
        let password = "hunter42";
        let hash_password = hash_password(password)?;
        assert_eq!(hash_password.len(), 97);
        assert!(verify_password(password, &hash_password)?);
        Ok(())
    }

    #[tokio::test]
    async fn create_and_verify_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("../migrations"),
        );

        let pool = tdb.get_pool().await;
        let email = "walden@gmail.org";
        let name = "walden";
        let password = "walden";
        let user = User::create(name, email, password, &pool).await?;
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, name);
        assert!(user.id > 0);

        let user = User::find_by_email(email, &pool).await?;
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, name);

        let user = User::verify(email, password, &pool).await?;
        assert!(user.is_some());

        Ok(())

    }
}