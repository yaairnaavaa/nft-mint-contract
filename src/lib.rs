use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::events::*;
pub use crate::royalty::*;

mod internal;
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod events;
mod royalty; 

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";
pub const ICON: &str = "data:image/jpeg;base64,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";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "NFT Mint Contract".to_string(),
                symbol: "NMC".to_string(),
                //icon: None,
                icon: Some(ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }
}

#[cfg(test)]
mod tests;