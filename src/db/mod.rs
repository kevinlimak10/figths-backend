mod functions;
pub mod model;
mod repo;

use crate::prelude::*;

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;


#[async_trait]
pub trait TransactionMgm {
    async fn create(&self, transaction: &Transaction) -> Result<&i32>;
    async fn updateTransaction(
        &self,
        id: &i32,
        status: &i32,
    ) -> Result<&i32>;

}

#[async_trait]
pub trait UsersMgm {
    async fn get(&self, id: &i32) -> Result<Option<User>>;
    async fn update_next_transaction(&self, id: &i32, id_transaction: &i32 ) -> Result<i32>;
    async fn get_transaction(
        &self,
        id: &i32
    ) -> Result<&User>;

    async fn get_next_transaction(
        &self,
        id: &i32
    ) -> Result<Option<&i32>>;
}

pub fn new_users(pool: PgPool) -> impl UsersMgm + Clone {
    repo::UsersMgmRepo::new(pool)
}

pub fn new_transaction(pool: PgPool) -> impl TransactionMgm + Clone {
    repo::TransactionMgmRepo::new(pool)
}
