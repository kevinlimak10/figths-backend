use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Error, MySqlConnection, MySqlPool, Result};
mod model;
pub async fn create(&self, transaction: &Transaction) -> Result<i32> {
    let mut conn = self.pool.acquire().await?;
    let result = sqlx::query!(
            "INSERT INTO transactions (description, type, price, status_id, id_user, insert_date) VALUES (?, ?, ?, ?, ?, ?)",
            transaction.description,
            transaction.type_,
            transaction.price,
            transaction.status_id,
            transaction.id_user,
            transaction.insert_date
        )
        .execute(&mut *conn)
        .await?;

    Ok(result.last_insert_id() as i32)
}
pub async fn update_transaction(&self, id: i32, status: i32) -> Result<i32> {
        let mut conn = self.pool.acquire().await?;
        let result = sqlx::query!(
            "UPDATE transactions SET status_id = ? WHERE id = ?",
            status,
            id
        )
        .execute(&mut *conn)
        .await?;

        Ok(result.rows_affected() as i32)
    }
pub async fn get(&self, id: i32) -> Result<Option<User>> {
        let mut conn = self.pool.acquire().await?;
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, balance, limit, current_transaction FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(user)
    }

    pub async fn update_next_transaction(&self, id: i32, id_transaction: i32) -> Result<i32> {
        let mut conn = self.pool.acquire().await?;
        let result = sqlx::query!(
            "UPDATE users SET current_transaction = ? WHERE id = ?",
            id_transaction,
            id
        )
        .execute(&mut *conn)
        .await?;

        Ok(result.rows_affected() as i32)
    }

    pub async fn get_transaction(&self, id: i32) -> Result<User> {
        let mut conn = self.pool.acquire().await?;
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, balance, limit, current_transaction FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(user)
    }

    pub async fn get_next_transaction(&self, id: i32) -> Result<Option<i32>> {
        let mut conn = self.pool.acquire().await?;
        let user = sqlx::query!(
            "SELECT current_transaction FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(user.current_transaction)
    }
