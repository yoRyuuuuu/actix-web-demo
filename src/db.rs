use crate::model::{Signup, User};
use sqlx::postgres::PgPool;

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> sqlx::Result<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT name, email, password, create_at, update_at FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(pool: &PgPool, user: &Signup) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        user.name,
        user.email,
        user.password
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
