use crate::*;
pub const IMAGE: &str = "QmQsrnCZ9uoYwqWS66CwhGQFEMGCMFtjntBisEGggaKL9o";

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, receiver_id: AccountId, metadata: TokenMetadata) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();
        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }

    #[payable]
    pub fn mint( &mut self, receiver_id: AccountId) -> String {
        let deposit = env::attached_deposit();

        // Verificar si el DAO es quien intenta minar el token, de lo contrario verificar las suscripciones
        let mut new_token = TokenMetadata {
            title:  Some("NFT #".to_string()+&self.token_metadata_by_id.len().to_string()), 
            description:  Some("Este NFT tiene metadata estática".to_string()),
            media:  Some("".to_string()),
            expires_at: None,
            starts_at: None,
            copies: None,
            extra: None,
            issued_at: None,
            media_hash: None,
            reference: None,
            reference_hash: None,
            updated_at: None
        };

        let initial_storage_usage = env::storage_usage();

        new_token.media = Some(IMAGE.to_string());
        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &new_token);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);


        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    
        return "El token fué minado con éxito".to_string();
    }

    pub fn nft_update(&mut self, nft_id: TokenId, new_metadata: TokenMetadata) -> String {
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&nft_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        if account_id.clone() != owner_id.clone().parse::<AccountId>().unwrap() {
            env::panic_str("El NFT no te pertenece");
        }

        self.token_metadata_by_id.insert(&nft_id, &new_metadata);

        "NFT Actualizado".to_string()

    }

}