use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub mint_authority: Pubkey,
    pub admin: Pubkey,
    pub freeze_authority: Pubkey,
    pub base_uri: String,
    pub seller_fee_basis_points: u16,
    pub image_url: String,
    pub max_total_supply: u64,
    pub wallet_mint_cap: u64,
    pub yearly_mint_cap: u64,
}

impl TokenConfig {
    pub fn new(mint_pubkey: Pubkey) -> Self {
        Self {
            name: "Meowdolf Kittler".to_string(),
            symbol: "KTLR".to_string(),
            decimals: 9,
            initial_supply: 1_000_000_000,
            mint_authority: mint_pubkey,
            admin: Pubkey::new_unique(),
            freeze_authority: Pubkey::new_unique(),
            base_uri: "https://example.com/api/tokens/".to_string(),
            seller_fee_basis_points: 500,
            image_url: "https://drive.google.com/file/d/1z17iO5THVHSd2vykIx8qbZFOQy0_-t97/view?usp=drive_link".to_string(),
            max_total_supply: 1_000_000_000_000,
            wallet_mint_cap: 10_000,
            yearly_mint_cap: 1_000_000_000,
        }
    }
}
