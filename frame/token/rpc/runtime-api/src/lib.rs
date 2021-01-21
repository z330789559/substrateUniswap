#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;


/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.

sp_api::decl_runtime_apis! {
    pub trait TokenApi<AccountId,Balance>  where 
    AccountId: Codec,
    Balance: Codec, {
        fn publish_token(account: AccountId,balance:Balance) -> u32;
    }
}