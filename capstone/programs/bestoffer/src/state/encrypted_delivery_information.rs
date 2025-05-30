use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EncryptedDeliveryInformation {
    pub nonce: [u8; 24],

    pub buyer_ephemeral_pubkey: [u8; 32],

    // max 100 characters × 4 bytes (UTF-8 worst case) + 16 bytes for crypto_box MAC = 416
    #[max_len(416)]
    pub encrypted_delivery_lastname: Vec<u8>,

    // max 100 characters × 4 bytes (UTF-8 worst case) + 16 bytes for crypto_box MAC = 416
    #[max_len(416)]
    pub encrypted_delivery_firstname: Vec<u8>,

    // max 150 characters × 4 bytes + 16 = 616
    #[max_len(616)]
    pub encrypted_delivery_address_line_1: Vec<u8>,

    // max 150 characters × 4 bytes + 16 = 616
    #[max_len(616)]
    pub encrypted_delivery_address_line_2: Option<Vec<u8>>,

    // max 100 characters × 4 bytes + 16 = 416
    #[max_len(416)]
    pub encrypted_delivery_city: Vec<u8>,

    // max 50 characters × 4 bytes + 16 = 216
    #[max_len(216)]
    pub encrypted_delivery_postal_code: Vec<u8>,

    // max 2 characters × 4 bytes + 16 = 24
    #[max_len(24)]
    pub encrypted_delivery_country_code: Vec<u8>,

    // max 50 characters × 4 bytes + 16 = 216
    #[max_len(216)]
    pub encrypted_delivery_state_code: Option<Vec<u8>>,
}
