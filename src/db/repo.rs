use crate::api::v1::{AuctionInfo, AuctionRec, AuctionStatus, BidInfo, BidRec};
use crate::prelude::*;
use async_trait::async_trait;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use super::query::{
    create, update_transaction, get, update_next_transaction, get_transaction, get_next_transaction
};
use super::model::{Transaction, User};
use super::{UsersMgm, TransactionMgm};



#[derive(Debug, Clone)]
pub struct TransactionMgmRepo {
    pool: Arc<PgPool>,
}

impl TransactionMgmRepo {
    pub fn new(pool: PgPool) -> Self {
        pool: Arc::new(pool)
    }
}


#[async_trait]
impl TransactionMgm for TransactionMgmRepo {

    async fn create(&self, transaction: &Transaction) -> Result<&i32> {
        let mut tx = self.pool.clone().begin().await?;
        let row = create(&mut tx, transaction).await?;
        tx.commit().await?;
        Ok(&i32::from(&row))
    }
    async fn update_transaction(
        &self,
        id: &i32,
        status: &i32,
    ) -> Result<&i32>{
        let mut tx = self.pool.clone().begin().await?;
        let row = update_transaction(&mut tx, transaction).await?;
        tx.commit().await?;
        Ok(&i32::from(&row))
    }

}


#[async_trait]
impl UsersMgm for UsersMgmRepo {
    async fn get(&self, id: &i32) -> Result<Option<User>>{
        let row = get(self.pool.as_ref(), id).await?;
        Ok(row.map(|row| User::from(&row)))
    }
    async fn update_next_transaction(&self, id: &i32, id_transaction: &i32 ) -> Result<i32>{
        let mut tx = self.pool.clone().begin().await?;
        let row = update_transaction(&mut tx, id, id_transaction).await?;
        tx.commit().await?;
        Ok(&i32::from(&row))
    }
    async fn get_transaction(
        &self,
        id_transaction: &i32
    ) -> Result<&User>{
        let row = get_transaction(self.pool.as_ref(), id).await?;
        Ok(&i32::from(&row))
    }

    async fn get_next_transaction(
        &self,
        id: &i32
    ) -> Result<Option<&i32>>{
        let row = get_next_transaction(self.pool.as_ref(), id).await?;
        Ok(&i32::from(&row))
    }

}