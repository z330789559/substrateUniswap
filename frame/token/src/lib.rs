#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::traits::{
    Member, One, Zero, AtLeast32Bit, MaybeSerializeDeserialize, CheckedAdd,
    StaticLookup, BlakeTwo256, Block as BlockT, IdentityLookup, Verify, IdentifyAccount, NumberFor, Saturating,AtLeast32BitUnsigned
};
use sp_runtime::sp_std::{cmp, result, mem, fmt::Debug, ops::BitOr, convert::Infallible};
use codec::{Codec, Encode, Decode};
use frame_support::{
	decl_module, decl_event, decl_storage, decl_error, ensure, Parameter,
	parameter_types, storage::child::ChildInfo,
	dispatch,
	traits::{OnUnbalanced, Currency, Get, Time, Randomness},
};
use frame_system::{ensure_signed, ensure_root};
use sp_runtime::print;
use sp_runtime::OpaqueExtrinsic;
use sp_runtime::MultiSignature;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
 pub type BalanceOf<T> = <T as Trait>::Balance;

 type AccountDataOf<T> =  pallet_balances::AccountData<<T as Trait>::Balance>;
// 	<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.

pub trait Trait: frame_system::Trait {
    /// The overarching event type.
     type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

     type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + From<u128> +
     MaybeSerializeDeserialize + Debug;

    type AssetId: Parameter + AtLeast32Bit + Default + Copy;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Balance = <T as Trait>::Balance,
        AssetId = <T as Trait>::AssetId,
    {
        NewToken(AssetId, AccountId, Balance),
        /// <from, to, amount>
        Transfer(AccountId, AccountId, Balance),
        /// <owner, spender, amount>
        Approval(AccountId, AccountId, Balance),
    }
);

decl_error! {
    /// Errors for the fungible pallet.
    pub enum Error for Module<T: Trait> {
        /// Overflow during creation.
        CreationOverflow,
        /// Attempted to transfer zero tokens.
        TransferZeroAmount,
        /// Insufficient funds to make transfer.
        InsufficientFunds,
        /// Insufficient allowance to spend on behalf of an account.
        InsufficientAllowance,
    }
}

decl_storage!(
    trait Store for Module<T: Trait> as token {
         TokenCount get(fn token_count): T::AssetId;
         Balances get(fn balance_of):   double_map hasher(twox_64_concat) T::AssetId, hasher(twox_64_concat) T::AccountId=> T::Balance;
         TotalSupply get(fn total_supply):  map hasher(opaque_blake2_256) T::AssetId => T::Balance;
        /// ERC20 compatible.
        /// Maps (id, owner, spender) => amount.
        /// 	/// Allowance
        Allowance get(fn allowances): double_map hasher(twox_64_concat) T::AssetId,  hasher(blake2_128_concat)  (T::AccountId, T::AccountId)  => T::Balance;
 
    }
);

decl_module!(
    pub struct Module<T: Trait> for enum Call where origin: T::Origin  {

        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10 ]
        pub fn debug_create_token(
            origin,
            #[compact] total_supply: T::Balance,
        ) -> dispatch::DispatchResult 
        {
            let sender = ensure_signed(origin)?;

            let _id = Self::create_token(sender, total_supply);
            // Inspecting variables
        
            Ok(())
        }

        #[weight = 10 ]
        pub fn transfer(
            origin,
            id: T::AssetId,
            destination: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: T::Balance,
        ) -> dispatch::DispatchResult
        {
            let sender = ensure_signed(origin)?;
            let recipient = T::Lookup::lookup(destination)?;

            ensure!(!amount.is_zero(), Error::<T>::TransferZeroAmount);

            Self::do_transfer(id, sender.clone(), recipient.clone(), amount)
        }

        #[weight = 10 ]
        pub fn transfer_from(
            origin,
            id: T::AssetId,
            from: <T::Lookup as StaticLookup>::Source,
            to: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: T::Balance,
        ) -> dispatch::DispatchResult
        {
            let sender = ensure_signed(origin)?;
            let owner = T::Lookup::lookup(from)?;
            let recipient = T::Lookup::lookup(to)?;

            ensure!(!amount.is_zero(), Error::<T>::TransferZeroAmount);
            // let allowed = <Allowance<T>>::get((id, owner.clone(), sender.clone()));
            let allowed = <Allowance<T>>::get(id, (owner.clone(),sender.clone()));
            ensure!(allowed >= amount.clone(), Error::<T>::InsufficientAllowance);

            Self::do_transfer(id, owner.clone(), recipient.clone(), amount.clone())?;

            <Allowance<T>>::mutate(id, (owner.clone(),sender.clone()), |allowed| {
                *allowed -= amount;
            });

            Ok(())
        }

        #[weight = 10 ]
        pub fn approve(
            origin,
            id: T::AssetId,
            spender: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: T::Balance,
        ) -> dispatch::DispatchResult
        {
            let sender = ensure_signed(origin)?;
            let a_spender = T::Lookup::lookup(spender)?;

            ensure!(!amount.is_zero(), Error::<T>::TransferZeroAmount);

            <Allowance<T>>::mutate(id, (sender.clone(),a_spender.clone()), |allowed| {
                *allowed += amount.clone();
            });

            Self::deposit_event(RawEvent::Approval(sender.clone(), a_spender.clone(), amount));

            Ok(())
        }

        #[weight = 10 ]
        pub fn debug_mint(
            origin,
            id: T::AssetId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> dispatch::DispatchResult
        {
            ensure_signed(origin)?;
            Self::mint(id, to, amount)
        }

        #[weight = 10 ]
        pub fn debug_burn(origin, id: T::AssetId, from: T::AccountId, amount: T::Balance) 
            -> dispatch::DispatchResult
        {
            ensure_signed(origin)?;
            Self::burn(id, from, amount)
        }
    }
);

impl<T: Trait> Module<T> {
    pub fn mint(id: T::AssetId, to: T::AccountId, amount: T::Balance)
        -> dispatch::DispatchResult
    {
        <Balances<T>>::mutate(id, to, |bal| {
            *bal += amount.clone();
        });

        <TotalSupply<T>>::mutate(id, |sup| {
            *sup += amount;
        });

        Ok(())
    }

    pub fn burn(id: T::AssetId, from: T::AccountId, amount: T::Balance)
        -> dispatch::DispatchResult
    {
        <Balances<T>>::mutate(id, from, |bal| {
            *bal -= amount.clone();
        });

        <TotalSupply<T>>::mutate(id, |sup| {
            *sup -= amount;
        });

        Ok(())
    }

    pub fn create_token(who: T::AccountId, total_supply: T::Balance)
        -> T::AssetId
    {
        let id =<TokenCount<T>>::get();
        // TODO: Watch for overflow here. PUZZLE: Find a good solution that doesn't
        // need to make this function return a result, which may be an anti-pattern.
        let next_id = id.checked_add(&One::one()).unwrap();
        
        <Balances<T>>::insert(id, who.clone(), total_supply);
        <TotalSupply<T>>::insert(id, total_supply);
        <TokenCount<T>>::put(next_id);

        Self::deposit_event(RawEvent::NewToken(id, who, total_supply));
    
        id
    }

    pub fn do_transfer(id: T::AssetId, from: T::AccountId, to: T::AccountId, amount: T::Balance)
        -> dispatch::DispatchResult
    {
        let from_balance = <Balances<T>>::get(id, from.clone());
        ensure!(
            from_balance >= amount.clone(),
            Error::<T>::InsufficientFunds,
        );

        <Balances<T>>::mutate(id, from.clone(), |balance| {
            *balance -= amount.clone();
        });
        <Balances<T>>::mutate(id, to.clone(), |balance| {
            *balance += amount.clone();
        });

        Self::deposit_event(RawEvent::Transfer(from.clone(), to.clone(), amount.clone()));

        Ok(())
    }
}