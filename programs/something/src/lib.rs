use anchor_lang::prelude::*;
// use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::{self, Mint, TokenAccount, Token, SetAuthority,ID as SPL_TOKEN_ID};
use anchor_spl :: associated_token:: {AssociatedToken};
use solana_program::program_option::{COption};


declare_id!("5Ff4tvaCetzZmxviCK5HfGb1RKqhJYdcqAozCqG5Wxje");

#[program]
pub mod something {
    use metaplex_token_metadata::utils::{assert_initialized, assert_owned_by ,assert_signer};
    use metaplex_token_metadata::state::{Metadata,Data};
    use metaplex_token_metadata::instruction::{update_metadata_accounts};
   
    use super::*;
    

    pub fn initialize(ctx: Context<Initialize>, _init_nonce:u8,_program_address:Pubkey) -> ProgramResult {
        let program_pda_account = &mut ctx.accounts.program_pda;
        program_pda_account.signerA = "true".to_string();
        Ok(())
    }
  
    pub fn lock(ctx: Context<Lock>, init_nonce:u8,_bump_for_metadata:u8) -> ProgramResult {

         fn assert_keys_equal(key1: Pubkey, key2: Pubkey) -> ProgramResult {
            if key1 != key2 {
                Err(ErrorCode::InvalidOwnerForMint.into())
            } else {
                Ok(())
            }
        }

        let user_mint_account = &ctx.accounts.mint_account;

        let token_account= &ctx.accounts.token_account;
        let metadata_account = &ctx.accounts.metadata_account;   

        let owner_wallet = &ctx.accounts.owner_wallet;
        
        let mint_metadata = &mut Metadata::from_account_info(&ctx.accounts.metadata_account)?;
        
        
        assert_eq!(token_account.amount,1);
        assert_keys_equal(user_mint_account.key(), token_account.mint)?;
        assert_keys_equal(token_account.owner, *owner_wallet.key)?;
        assert_owned_by(metadata_account, &ctx.accounts.token_metadata_program.key)?;

        if !owner_wallet.to_account_info().is_signer {
            return Err(ErrorCode::OwnerMustBeSigner.into())
        }

        msg!("is mutable? {}", mint_metadata.is_mutable);
        if mint_metadata.is_mutable == false {
            return Err(ErrorCode :: AlreadyLocked.into())
        }

         


        

        msg!("is mutable? after {}", mint_metadata.is_mutable);
        // let new_data = Data {
        //     name: mint_metadata.data.name.clone(),
        //     symbol: mint_metadata.data.symbol.clone(),
        //     uri: "https://ifgv27za22gw26hddoplvrj6yp6jn2wprr26oqe3wu6hsoqins3q.arweave.net/QU1dfyDWjW144xueusU-w_yW6s-MdedAm7U8eToIbLc".to_string(),
        //     seller_fee_basis_points: mint_metadata.data.seller_fee_basis_points,
        //     creators: mint_metadata.data.creators.clone(),
        // }; 

        let new_data = Data {
            name: "scott  has ovaries!!".to_string(),
            symbol: "crazy".to_string(),
            uri: "https://ifgv27za22gw26hddoplvrj6yp6jn2wprr26oqe3wu6hsoqins3q.arweave.net/QU1dfyDWjW144xueusU-w_yW6s-MdedAm7U8eToIbLc".to_string(),
            seller_fee_basis_points: 500,
            creators: None
        }; 
        let metadata_infos = [
            ctx.accounts.metadata_account.clone(),
            ctx.accounts.program_pda_signer.to_account_info()

        ];
        let update_auth_seeds = [
            "new_update_auth".as_bytes(),
            
            &[init_nonce]
        ];
       


        anchor_lang::solana_program::program::invoke_signed(
            &update_metadata_accounts(
                *ctx.accounts.token_metadata_program.key, 
               *ctx.accounts.metadata_account.key,
               * ctx.accounts.program_pda_signer.to_account_info().key,
            None,
            Some(new_data),
            None,
             
                ), 

               &metadata_infos,

              &[&update_auth_seeds],
        
        )?;
     


       
        Ok(())
    }
}


//Token account and mint is taken in with constraints as seeds 
//Creating  ['metaplex', metaplex_program_id, mint_id].
#[derive(Accounts)]
#[instruction(my_bump:u8,program_address: Pubkey)]
pub struct Initialize<'info> {
    


#[account( 
    init,
    seeds = [
    "new_update_auth".as_bytes(), //Mark possible attack vector
    ],
    bump = my_bump,
    payer = owner_wallet,
    space = 400
)]  
pub program_pda: Account<'info,SignerA>,


#[account(mut)]
pub owner_wallet: Signer<'info>,
pub system_program: Program<'info, System>,




}

#[derive(Accounts)]
#[instruction(my_bump:u8, bump_for_metadata:u8)]
pub struct Lock<'info> {
    #[account(address = metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    pub mint_account: Account<'info,Mint>,
    pub owner_wallet: Signer<'info>, //any random wallet that is signing
    pub token_account: Account<'info,TokenAccount>,

    #[account(seeds = ["new_update_auth".as_bytes()],bump=my_bump)]
    pub program_pda_signer: Account<'info,SignerA>,

//    #[account(seeds =[owner_wallet.key.as_ref(),ID.as_ref(),mint_account.key().as_ref()],bump = bump_for_associated)]
//     pub associated_token_account_for_mint: AccountInfo<'info>,

    

    #[account(
        mut, 
        // seeds = ["metaplex".as_bytes(),
        // token_metadata_program.key.as_ref(),
        // mint_account.key().as_ref()],
        // bump = bump_for_metadata
    )]
    pub metadata_account: AccountInfo<'info>,  //possible flag here

    



    pub system_program: Program<'info, System>,
   
}

#[account]
pub struct SignerA{
    pub signerA: String,
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid owner for mint")]
    InvalidOwnerForMint,

    #[msg("Account must be signer")]
    OwnerMustBeSigner,

    #[msg("You NFT is already locked")]
    AlreadyLocked
}
