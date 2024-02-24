use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, env, Promise};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ArtMarketplace {
    listings: UnorderedMap<AccountId, ArtListing>,
    marketplace_balance: u128,
    commission_rate: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ArtListing {
    artist: AccountId,
    original_image_hash: Vec<u8>,
    glazed_image_hash: Vec<u8>,
    price: u128,
}

#[near_bindgen]
impl ArtMarketplace {
    #[init]
    pub fn new(commission_rate: u128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            listings: UnorderedMap::new(b"l"),
            marketplace_balance: 0,
            commission_rate,
        }
    }

    pub fn list_art(&mut self, artist: AccountId, original_image_hash: Vec<u8>, glazed_image_hash: Vec<u8>, price: u128) {
        let listing = ArtListing {
            artist: artist.clone(),
            original_image_hash,
            glazed_image_hash,
            price,
        };
        self.listings.insert(&artist, &listing);
    }

    #[payable]
    pub fn buy_art(&mut self, artist_addr: AccountId) {
        let listing = self.listings.get(&artist_addr).expect("Listing not found");
        let price = listing.price;
        
        assert!(env::attached_deposit() >= price, "Attached deposit is less than the price");

        let commission = (price * self.commission_rate) / 100;
        let artist_share = price - commission;

        self.marketplace_balance += commission;
        
        Promise::new(artist_addr.clone()).transfer(artist_share);
        
        self.listings.remove(&artist_addr);
    }

    pub fn withdraw_commission(&mut self, owner: AccountId) {
        assert_eq!(env::predecessor_account_id(), owner, "Only the owner can withdraw");
        
        Promise::new(owner).transfer(self.marketplace_balance);
        self.marketplace_balance = 0;
    }
}
