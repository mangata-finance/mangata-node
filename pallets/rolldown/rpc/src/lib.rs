// Copyright (C) 2021 Mangata team

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use rolldown_runtime_api::RolldownRuntimeApi;
pub use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

#[rpc(client, server)]
pub trait RolldownApi<BlockHash> {
	/// Calculates amount of available native rewards
	///
	/// * `account` - user account address
	/// * `liquidity_token` - liquidity token id
	/// * `at` - optional block hash
	#[method(name = "rolldown_pending_updates_hash")]
	fn pending_updates_hash(&self, at: Option<BlockHash>) -> RpcResult<sp_core::H256>;

	#[method(name = "rolldown_pending_updates")]
	fn pending_updates(&self, at: Option<BlockHash>) -> RpcResult<Vec<u8>>;
}

pub struct Rolldown<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, P> Rolldown<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

#[async_trait]
impl<C, Block> RolldownApiServer<<Block as BlockT>::Hash> for Rolldown<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: RolldownRuntimeApi<Block>,
{
	fn pending_updates_hash(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<sp_core::H256> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or(self.client.info().best_hash);
		api.get_pending_updates_hash(at).map_err(|e| {
			JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
				1,
				"Unable to serve the request",
				Some(format!("{:?}", e)),
			)))
		})
	}

	fn pending_updates(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Vec<u8>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or(self.client.info().best_hash);
		api.get_pending_updates(at).map_err(|e| {
			JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
				1,
				"Unable to serve the request",
				Some(format!("{:?}", e)),
			)))
		})
	}
}