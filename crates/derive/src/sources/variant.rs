//! Data source

use crate::{
    sources::{BlobSource, CalldataSource},
    traits::{AsyncIterator, BlobProvider, ChainProvider},
    types::StageResult,
};
use alloc::boxed::Box;
use alloy_primitives::Bytes;
use async_trait::async_trait;

/// An enum over the various data sources.
#[derive(Debug, Clone)]
pub enum EthereumDataSourceVariant<CP, B>
where
    CP: ChainProvider + Send,
    B: BlobProvider + Send,
{
    /// A calldata source.
    Calldata(CalldataSource<CP>),
    /// A blob source.
    Blob(BlobSource<CP, B>),
}

#[async_trait]
impl<CP, B> AsyncIterator for EthereumDataSourceVariant<CP, B>
where
    CP: ChainProvider + Send,
    B: BlobProvider + Send,
{
    type Item = Bytes;

    async fn next(&mut self) -> StageResult<Self::Item> {
        match self {
            EthereumDataSourceVariant::Calldata(c) => c.next().await,
            EthereumDataSourceVariant::Blob(b) => b.next().await,
        }
    }
}
