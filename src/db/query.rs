use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Error, MySqlConnection, MySqlPool, Result};

mod model;

pub async fn create(tx: &mut Transaction<'_, MySqlConnection>, transaction: &Transaction) -> Result<i32> {
    let result = sqlx::query!(
            "INSERT INTO transactions (description, type, price, status_id, id_user, insert_date) VALUES (?, ?, ?, ?, ?, ?)",
            transaction.description,
            transaction.type_,
            transaction.price,
            transaction.status_id,
            transaction.id_user,
            transaction.insert_date
        )
        .execute(&mut *tx)
        .await?;

    Ok(result.last_insert_id() as i32)
}

pub async fn update_transaction(tx: &mut Transaction<'_, MySqlConnection>, id: i32, status: i32) -> Result<i32> {
    let result = sqlx::query!(
            "UPDATE transactions SET status_id = ? WHERE id = ?",
            status,
            id
        )
        .execute(&mut *tx)
        .await?;

    Ok(result.rows_affected() as i32)
}

pub async fn get(tx: &mut Transaction<'_, MySqlConnection>, id: i32) -> Result<Option<User>> {
    let user = sqlx::query_as!(
            User,
            "SELECT id, name, balance, limit, current_transaction FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&mut *tx)
        .await?;

    Ok(user)
}

pub async fn update_next_transaction(tx: &mut Transaction<'_, MySqlConnection>, id: i32, id_transaction: i32) -> Result<i32> {
    let result = sqlx::query!(
            "UPDATE users SET current_transaction = ? WHERE id = ?",
            id_transaction,
            id
        )
        .execute(&mut *tx)
        .await?;

    Ok(result.rows_affected() as i32)
}

pub async fn get_transaction(tx: &mut Transaction<'_, MySqlConnection>, id: i32) -> Result<User> {

    let user = sqlx::query_as!(
            User,
            "SELECT id, name, balance, limit, current_transaction FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut *tx)
        .await?;

    Ok(user)
}

pub async fn get_next_transaction(tx: &mut Transaction<'_, MySqlConnection>, id: i32) -> Result<Option<i32>> {
    let user = sqlx::query!(
            "SELECT current_transaction FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut *tx)
        .await?;

    Ok(user.current_transaction)
}
