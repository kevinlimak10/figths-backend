extern crate rust_decimal;
use rust_decimal::Decimal;


#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Transaction {
    pub id: Option<i32>,
    pub description: String,
    pub type: String,
    pub price: Decimal,
    pub status_id: i32,
    pub id_user: i32,
    pub insert_date: Option<DateTime<chrono::Utc>>
}

pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub balance: Decimal,
    pub limit: Decimal,
    pub current_transaction: Optional<i32>

}