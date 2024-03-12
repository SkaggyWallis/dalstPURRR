/*
MIT License

Copyright (c) [year] [author]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

//      ~-`~-`~-`~-MEOWDOLF-KITTLER`~-`~-`~-`~
//`~-`~-`~-`~-`~-`~-`~-`~$KTLR~-`~-`~-`~-~-`~-`~-`~-
//`~-`~-`~-`~-`~-`~-~TOKEN PROGRAM~-`~-`~-`~-~-`~-`~
//`~-`~-`~-`~-`~-`~-IN FUD WE TROLL~-`~-`~-`~-~-`~-`


use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::Pack,
    sysvar::{rent::Rent, Sysvar},
    clock::Clock,
};

use spl_token_2022::{Mint, TokenAccount};

#[derive(BorshSerialize, BorshDeserialize)]  
struct TokenConfig {
  pub name: String,
  pub symbol: String, 
  pub decimals: u8,
  pub mint_authority: Pubkey,
  pub initial_supply: u64,

  pub freeze_authority: Option<Pubkey>,
  pub base_uri: String,
  pub seller_fee_basis_points: u16,
                // other advanced fields
}

                //Tracks Wallet Minting activity
#[derive(BorshSerialize, BorshDeserialize)]
struct WalletMintTracker {
  address: Pubkey,
  tokens_minted: u64,
  last_mint_time: i64
} 


const WALLET_MINT_CAP: u64 = 10_000;

mod instructions {

  // FUNCTIONS


  pub fn transfer(
    from: &TokenAccount, 
    to: &TokenAccount,
    amount: u64
  ) -> ProgramResult {
    // transfer logic
  }

}


fn initialize(config: TokenConfig, accounts: &[AccountInfo]) -> ProgramResult {
    let mint = Mint {
        mint_authority: config.mint_authority,
        supply: config.initial_supply,
                // Add other fields
    };

    Mint::pack(mint, &mut accounts[0].data.borrow_mut())?;

                // Save config to upgradable account
                // Save other necessary information...

    Ok(())
}

fn mint_initial_supply(config: &TokenConfig, accounts: &[AccountInfo]) -> ProgramResult {
    let mint_account = &accounts[0];
    let token_account = &accounts[1];

    mint_account.mint_to(token_account, config.initial_supply)?;

    Ok(())
}


fn mint_tokens() {

  // Existing code

  let supply = get_current_supply(accounts);
  
  // Increment supply
  supply += amount;

  // Update mint account

  // EVENT
  emit!(TokenMintEvent {...});

  instructions::transfer(&mint_account, &token_account, amount);
  
  // More existing code

}

// Leave existing function signatures
fn get_current_supply() {} 

fn get_wallet_tracker() {}


  Ok(())
}
  emit!(TokenMintEvent {
    amount,
    authority: mint_authority
  });

}

fn burn_tokens(config: &TokenConfig, accounts: &[AccountInfo], amount: u64) -> ProgramResult {

                // Burn logic
                // Update supply

                // Emit burn event

  Ok(())  
}

fn transfer(from: &TokenAccount, to: &TokenAccount, amount: u64) -> ProgramResult {

  from.balance -= amount;
  to.balance += amount;

  emit!(TokenTransferEvent {
    amount,
    from: from.address,
    to: to.address
  });

  Ok(())
}


    tracker.tokens_minted += amount;
    tracker.last_mint_time = Clock::get()?.unix_timestamp;

                // Store updated tracker account
                // Save other necessary information...

    Ok(())
}

fn burn_tokens(/*...*/) -> ProgramResult {
                // Burn logic...
                // Implement the burn logic and necessary checks

    Ok(())
}

fn pause_minting(/*...*/) -> ProgramResult {
                // Pause logic...
                // Implement the pause logic

    Ok(())
}

fn upgrade(/*...*/) -> ProgramResult {
                // Upgrade logic...
                // Implement the upgrade logic

    Ok(())
}

fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = deserialize_instruction(input)?;

match instruction {

  Transfer(from, to, amount) => {
    instructions::transfer(from, to, amount)?;
  }

}
    

match instruction {
  MintTokens(amount) => {
    mint_tokens(&config, accounts, amount)?;
  }
  BurnTokens(amount) => {
    burn_tokens(&config, accounts, amount)?;
  }
                //...
}


    match instruction {
        TokenInstruction::Initialize(config) => {
            initialize(config, accounts)?;
        }
        TokenInstruction::Mint(amount) => {
            let config = get_config(accounts)?;
            mint_tokens(&config, accounts, amount)?;
        }
        TokenInstruction::Burn(amount) => {
            burn_tokens(/*...*/)?;
        }
        TokenInstruction::PauseMinting => {
            pause_minting(/*...*/)?;
        }
        TokenInstruction::Upgrade => {
            upgrade(/*...*/)?;
        }
                // Add more cases for other instructions if needed
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

                // Entry point
entrypoint! {
    fn entry(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        process(program_id, accounts, input)
    }
}

                // Helper functions
fn get_current_supply(accounts: &[AccountInfo]) -> u64 {
  let mint_account = &accounts[0]; // Token mint account
  let mint_data = &mint_account.data.borrow();
  let mint = Mint::unpack(mint_data).unwrap();

  mint.supply // Return supply field
}

fn get_wallet_tracker(accounts: &[AccountInfo]) -> WalletTracker {
  let tracker_account = &accounts[1]; // Tracker account 
  let tracker_data = &tracker_account.data.borrow();
  
  WalletTracker::unpack(tracker_data).unwrap() // Unpack and return
}



fn get_current_supply(/*...*/) -> u64 {
                // Implementation
}

  let tracker_account = &accounts.find_wallet(wallet).unwrap();
  let tracker_data = &tracker_account.data;

  WalletMintTracker::unpack(tracker_data)?
}

                // Mint tokens 
fn mint_tokens(/*...*/) {

  let tracker = get_wallet_tracker(accounts, &config.mint_authority);
  
  let supply = get_current_supply(accounts);

                 // Rest of function
}

  config: &TokenConfig,
  accounts: &[AccountInfo],
  amount: u64  
) -> ProgramResult {

  let supply = get_current_supply(accounts);

  if supply + amount > TOTAL_SUPPLY {
    return Err(ErrorCode::AnnualCapExceeded);
  }

  let tracker = get_wallet_tracker(accounts, &config.mint_authority);

  if tracker.tokens_minted + amount > WALLET_CAP {
    return Err(ErrorCode::WalletCapExceeded);
  }  

            // Mint new tokens
  let mint_account = &accounts.find(MINT_PUBKEY).unwrap();
  mint_account.mint_to(token_account, amount)?;

            // Update tracker  
  let tracker_account = &accounts.find(tracker.pubkey).unwrap();
  tracker_account.tokens_minted += amount;

            // Save updated accounts
  token_account.save()?;
  tracker_account.save()?;

  Ok()
}

}



  mint_tokens(&config, accounts, 100);
}

            // Configuration for initializing the token
#[derive(BorshSerialize, BorshDeserialize)]
struct TokenConfig {
  mint_authority: Pubkey,
  admin: Pubkey,
  upgrade_authority: Pubkey,
}

const TOTAL_SUPPLY: u64 = 100_000_000_000;
const INITIAL_MINT: u64 = 1_000_000_000;
const ANNUAL_MINT_CAP: u64 = 1_000_000_000;
const WALLET_MINT_CAP: u64 = 10_000;
const LAMPORTS_PER_USDC: u64 = 1000000;
const TOTAL_SUPPLY: u64 = 100_000_000; 
const MINT_PUBKEY: Pubkey = Pubkey::new_unique();

fn calculate_fee(amount: u64) -> u64 {

  let tokens_to_mint = 0;

  if amount == 10 * LAMPORTS_PER_USDC {
    tokens_to_mint = 100; 
  } else if amount == 20 * LAMPORTS_PER_USDC {
    tokens_to_mint = 250;
  } else if amount == 40 * LAMPORTS_PER_USDC {
    tokens_to_mint = 667;
  } else if amount == 60 * LAMPORTS_PER_USDC {
    tokens_to_mint = 1500;
  } else if amount == 80 * LAMPORTS_PER_USDC {
    tokens_to_mint = 2667;
  } else if amount == 100 * LAMPORTS_PER_USDC {
    tokens_to_mint = 10000;
  }

  tokens_to_mint
}


            // Initializes the token mint and accounts
                ///
            // # Arguments
                ///
            // * `config` - Token configuration object
            // * `accounts` - Accounts required for initialization 
    fn initialize(config: TokenConfig, accounts: &[AccountInfo]) -> ProgramResult {
    let mint = Mint {
        mint_authority: config.mint_authority,
        supply: config.initial_supply,
        // other fields
    };

    Mint::pack(mint, &mut accounts[0].data.borrow_mut())?;

            // Save config to upgradable account

    Ok(())
}

fn mint_initial_supply(config: &TokenConfig, accounts: &[AccountInfo]) -> ProgramResult {
    let mint_account = &accounts[0];
    let token_account = &accounts[1];

    mint_account.mint_to(token_account, config.initial_supply)?;

    Ok(())
}

fn mint_tokens(
    config: &TokenConfig,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Access image
    let image = &config.token_image;

    // ...

    // Check supply limit
    require!(supply + amount <= MAX_SUPPLY, SupplyLimitExceeded);

    // Check mint authority
    let mint_authority = &config.mint_authority;
    require!(mint_authority == &accounts.mint_authority, InvalidMintAuthority);

    // Check token account owner
    let token_account = &accounts.token_account;
    require!(token_account.owner == mint_authority, InvalidTokenAccountOwner);

    // Check wallet cap
    let tracker = get_wallet_tracker(accounts, mint_authority);
    require!(tracker.tokens_minted + amount <= WALLET_CAP, WalletCapExceeded);

    // Mint token logic
    token_account.balance += amount;
    supply += amount;

    // Update tracker
    tracker.tokens_minted += amount;

    // Emit event
    emit!(TokenMintEvent {
        amount,
        to: token_account.address,
    });

    Ok(())
}

// ...

fn mint_tokens_handler(config: &TokenConfig, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let supply = get_current_supply(accounts)?;

    // Check annual cap
    if supply + amount > TOTAL_SUPPLY {
        return Err(ErrorCode::AnnualCapExceeded);
    }

    // Mint new tokens
    let mint_account = &accounts.find(MINT_PUBKEY).unwrap();
    let token_account = &accounts.find(TOKEN_ACCOUNT_PUBKEY).unwrap(); // Replace TOKEN_ACCOUNT_PUBKEY with the actual pubkey for the token account
    mint_account.mint_to(token_account, amount)?;

    // Update tracker
    let tracker_account = &accounts.find(TRACKER_PUBKEY).unwrap(); // Replace TRACKER_PUBKEY with the actual pubkey for the tracker account
    tracker_account.tokens_minted += amount;

    // Save updated accounts
    token_account.save()?;
    tracker_account.save()?;

    // Call mint_tokens function
    mint_tokens(config, accounts, amount)?;
    Ok(())
}


// ...


// Burn tokens instruction
fn burn_tokens(accounts: &[AccountInfo], amount: u64, config: &TokenConfig) -> ProgramResult {
    let token_account = &accounts[0];  // existing code
    let mint_account = &accounts[1];   // existing code
    let tracker_account = &accounts[2]; // existing code

    // Check if burning is allowed based on the date
    let clock = Clock::get()?;
    if !is_burn_allowed(clock) {
        return Err(TokenError::BurnNotAllowed.into());
    }

    // Check if the burn amount is within the limits
    let token_amount = token_account.amount; // existing code
    if amount > token_amount {
        return Err(TokenError::InsufficientFunds.into());
    }

    // Calculate the burn amount (1% of total supply)
    let total_supply = mint_account.supply; // existing code
    let burn_amount = total_supply / 100;
    if amount != burn_amount {
        return Err(TokenError::InvalidBurnAmount.into());
    }

    // Perform burn logic
    token_account.amount -= amount;  // existing code
    mint_account.supply -= amount;   // existing code
    tracker_account.tokens_burned += amount; // existing code

    // Save updated accounts
    TokenAccount::pack(token_account, &mut accounts[0].data.borrow_mut())?;
    Mint::pack(mint_account, &mut accounts[1].data.borrow_mut())?;
    WalletMintTracker::pack(tracker_account, &mut accounts[2].data.borrow_mut())?;

    // Emit burn event
    emit!(TokenBurnEvent {
        amount,
        from: token_account.address,
    });

    Ok(())
}

// Helper function to check if burning is allowed on a specific date
fn is_burn_allowed(clock: Clock) -> bool {
    // Add your logic to determine if burning is allowed on the given date
    // For example, allow burning only on April 20
    clock.unix_timestamp == APRIL_20_TIMESTAMP
}

// Constants
const APRIL_20_TIMESTAMP: i64 = /* Set the timestamp for April 20 */;


 // April 20 timestamp helpers
const FIRST_BURN_DATE: i64 = /* Set the first April 20 timestamp */;

fn is_burn_allowed(clock: Clock) -> bool {
    let current = clock.unix_timestamp;

    if current < FIRST_BURN_DATE {
        return false;
    }

    current >= april_20_this_year(clock) && current < april_20_next_year(clock)
}

fn april_20_this_year(clock: Clock) -> i64 {
    // Logic to get this year's Apr 20 timestamp
    // Example: Replace with your actual logic to calculate Apr 20 this year
    clock.unix_timestamp
}

fn april_20_next_year(clock: Clock) -> i64 {
    // Logic to get next year's Apr 20 timestamp
    // Example: Replace with your actual logic to calculate Apr 20 next year
    clock.unix_timestamp + 365 * 24 * 60 * 60 // Adding one year in seconds
}

// Other functions...

fn pause_minting(/*...*/) -> ProgramResult {
    // Pause logic...
}

fn upgrade(/*...*/) -> ProgramResult {
    // Upgrade logic...
}

fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = deserialize_instruction(input)?;

    match instruction {
        TokenInstruction::Initialize(config) => {
            initialize(config, accounts);
        }
        TokenInstruction::Mint(amount) => {
            let config = get_config(accounts)?;
            mint_tokens(&config, accounts, amount);
        }
        // Add more cases for other instructions if needed
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

// Entry point
entrypoint! {
    fn entry(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        process(program_id, accounts, input)
    }
}

// Additional helper functions...

// Deserialize instruction function
fn deserialize_instruction(input: &[u8]) -> Result<TokenInstruction, ProgramError> {
    // Implement your deserialization logic here
}

// Get config function
fn get_config(accounts: &[AccountInfo]) -> Result<TokenConfig, ProgramError> {
    // Implement your logic to retrieve the config from accounts
}

// Other imports and definitions...

// Continue with the rest of your code...

use spl_token_2022::{Mint, TokenAccount}; 

                // Instruction enum
enum TokenInstruction {
  Initialize,
  Mint,
  Burn,
  Transfer,
                // Other instructions   
}

                // Config struct

                // Other fields
}

                // Deserialize instruction
fn deserialize_instruction(input: &[u8]) -> Result<TokenInstruction, ProgramError> {
  let instruction = TokenInstruction::try_from_slice(input)?;
  Ok(instruction)  
}

                // Get config account
fn get_config(accounts: &[AccountInfo]) -> Result<TokenConfig> {
  let config_account = &accounts.config_account;
  let config = TokenConfig::unpack(&config_account.data.borrow())?;
  Ok(config)
}

                // Initialize  
fn initialize(config: TokenConfig, accounts: &[AccountInfo]) -> ProgramResult {

  let mint = Mint {
                // Initialize mint  
  };

                // Create token account for initial supply

                // Save config to upgradable account

                // Emit initialization event

}
 

  let config_account = &mut accounts.config_account;
  config_account.data = config.try_to_vec()?;

  Ok(())
}

                // Mint tokens
fn mint_tokens(config: &TokenConfig, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
  
                // Mint logic

                // Save accounts
   accounts[TOKEN_ACCOUNT_INDEX].save()?;
    accounts[TRACKER_ACCOUNT_INDEX].save()?;

    Ok(())
}
// 

                // Entrypoint
entrypoint!(process_instruction);
fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  input: &[u8],
) -> ProgramResult {

  let instruction = deserialize_instruction(input)?;

  match instruction {
    TokenInstruction::Initialize(config) => {
      initialize(config, accounts)?;
    }
    TokenInstruction::Mint(amount) => {
      let config = get_config(accounts)?;
      mint_tokens(&config, accounts, amount)?;
    }
  }
  
  Ok(())
}

// Entry point
entrypoint! {
    fn entry(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        process(program_id, accounts, input)
    }
}

// Additional helper functions...

// Deserialize instruction function
fn deserialize_instruction(input: &[u8]) -> Result<TokenInstruction, ProgramError> {
    let instruction = TokenInstruction::try_from_slice(input)?;
    Ok(instruction)
}

// Get config function
fn get_config(accounts: &[AccountInfo]) -> Result<TokenConfig, ProgramError> {
    let config_account = &accounts[CONFIG_ACCOUNT_INDEX];
    let config = TokenConfig::unpack(&config_account.data.borrow())?;
    Ok(config)
}

// Continue with the rest of your code...

use spl_token_2022::{Mint, TokenAccount};

// Instruction enum
enum TokenInstruction {
    Initialize,
    Mint,
    Burn,
    Transfer,
    // Other instructions
}

// Config struct
#[derive(BorshSerialize, BorshDeserialize)]
struct TokenConfig {
    // Fields in TokenConfig
    // ...
}

// Deserialize instruction
fn deserialize_instruction(input: &[u8]) -> Result<TokenInstruction, ProgramError> {
    let instruction = TokenInstruction::try_from_slice(input)?;
    Ok(instruction)
}

// Get config account
fn get_config(accounts: &[AccountInfo]) -> Result<TokenConfig, ProgramError> {
    let config_account = &accounts[CONFIG_ACCOUNT_INDEX];
    let config = TokenConfig::unpack(&config_account.data.borrow())?;
    Ok(config)
}

// Initialize
fn initialize(config: TokenConfig, accounts: &[AccountInfo]) -> ProgramResult {
    let mint = Mint {
        // Initialize mint
        // ...
    };

    // Create token account for initial supply
    // ...

    // Save config to upgradable account
    let config_account = &mut accounts[CONFIG_ACCOUNT_INDEX];
    config_account.data = config.try_to_vec()?;

    // Emit initialization event
    emit!(InitializationEvent { /* ... */ });

    Ok(())
}

// Mint tokens
fn mint_tokens(config: &TokenConfig, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // Mint logic
    // ...

    // Save accounts
    accounts[TOKEN_ACCOUNT_INDEX].save()?;
    accounts[TRACKER_ACCOUNT_INDEX].save()?;

    Ok(())
}

// Entrypoint
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = deserialize_instruction(input)?;

    match instruction {
        TokenInstruction::Initialize(config) => {
            initialize(config, accounts)?;
        }
        TokenInstruction::Mint(amount) => {
            let config = get_config(accounts)?;
            mint_tokens(&config, accounts, amount)?;
        }
        // Add more cases for other instructions if needed
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}