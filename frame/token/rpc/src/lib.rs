use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use std::sync::Arc;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT,MaybeDisplay, MaybeFromStr},
};
use codec::{Codec, Decode, Encode};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
pub use pallet_token_rpc_runtime_api::{
    self as runtime_api, TokenApi as TokenRunApi }; 


#[rpc]    
pub trait TokenApi<BlockHash, AccountId, Balance>
 {

    
    #[rpc(name = "publish_token")]
    fn publish_token(&self,	at: Option<BlockHash>,account: AccountId,balance:Balance) -> Result<u32>;
}


// An implementation of contract specific RPC methods.
pub struct Tokens<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> Tokens<C, B> {
	/// Create new `Contracts` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Tokens {
			client,
			_marker: Default::default(),
		}
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}
impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, AccountId, Balance> TokenApi<
		<Block as BlockT>::Hash,
		AccountId,Balance> for Tokens<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: TokenRunApi<Block,AccountId,Balance>,
	AccountId: Codec,
	Balance: Codec,
{

    fn publish_token(&self,	at: Option<<Block as BlockT>::Hash>,account: AccountId,balance:Balance) -> Result<u32>{
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));
		api.publish_token(&at,account,balance).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to publish_token dispatch info.".into(),
			data: Some(format!("{:?}", e).into()),
		})
    }
}